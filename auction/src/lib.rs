use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    program::{invoke, invoke_signed},
    system_instruction,
    clock::Clock,
};
use borsh::{BorshDeserialize, BorshSerialize};

// NFT 元数据结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct NFTState {
    token_id: u64,        // 唯一标识
    owner: Pubkey,        // 所有者公钥
    metadata_uri: String, // 元数据 URI
}

// 拍卖状态结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct AuctionState {
    nft_account: Pubkey,   // NFT 账户
    seller: Pubkey,        // 卖家公钥
    highest_bid: u64,      // 最高出价
    highest_bidder: Pubkey,// 最高出价者公钥
    end_time: i64,         // 拍卖结束时间（Unix 时间戳）
    active: bool,          // 拍卖是否有效
}

// 指令枚举
#[derive(BorshSerialize, BorshDeserialize)]
enum AuctionInstruction {
    StartAuction { token_id: u64, starting_bid: u64, duration: i64 },
    PlaceBid { bid_amount: u64 },
    CloseAuction,
}

// 入口函数
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = AuctionInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        AuctionInstruction::StartAuction { token_id, starting_bid, duration } => {
            msg!("Starting auction for NFT with token_id: {}, starting bid: {}", token_id, starting_bid);
            let nft_account = next_account_info(accounts_iter)?;
            let seller = next_account_info(accounts_iter)?;
            let auction_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let clock = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !seller.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if nft_account.owner != program_id || auction_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"auction", &token_id.to_le_bytes()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let rent = Rent::get()?;
            if !rent.is_exempt(auction_account.lamports(), auction_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }

            // 验证 NFT 所有者
            let mut nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            if nft_state.token_id != token_id || nft_state.owner != *seller.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // 初始化拍卖状态
            let clock = Clock::get()?;
            let auction_state = AuctionState {
                nft_account: *nft_account.key,
                seller: *seller.key,
                highest_bid: starting_bid,
                highest_bidder: Pubkey::default(),
                end_time: clock.unix_timestamp + duration,
                active: true,
            };
            auction_state.serialize(&mut &mut auction_account.data.borrow_mut()[..])?;

            // 转移 NFT 至 PDA 托管
            nft_state.owner = pda_key;
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;
            Ok(())
        }
        AuctionInstruction::PlaceBid { bid_amount } => {
            msg!("Placing bid: {} Lamports", bid_amount);
            let nft_account = next_account_info(accounts_iter)?;
            let bidder = next_account_info(accounts_iter)?;
            let auction_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let previous_bidder = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !bidder.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if auction_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let mut auction_state = AuctionState::try_from_slice(&auction_account.data.borrow())?;
            if !auction_state.active || auction_state.nft_account != *nft_account.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"auction", &auction_state.nft_account.to_bytes()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // 验证拍卖未结束
            let clock = Clock::get()?;
            if clock.unix_timestamp > auction_state.end_time {
                return Err(ProgramError::InvalidAccountData);
            }

            // 验证出价高于当前最高出价
            if bid_amount <= auction_state.highest_bid {
                return Err(ProgramError::Custom(1)); // 出价过低
            }

            // 退还前最高出价者的 Lamports
            if auction_state.highest_bidder != Pubkey::default() {
                let refund_instruction = system_instruction::transfer(
                    &pda_key,
                    &auction_state.highest_bidder,
                    auction_state.highest_bid,
                );
                invoke_signed(
                    &refund_instruction,
                    &[pda.clone(), previous_bidder.clone(), system_program.clone()],
                    &[&[b"auction", &auction_state.nft_account.to_bytes(), &[bump]]],
                )?;
            }

            // 转移新出价至 PDA
            let bid_instruction = system_instruction::transfer(
                bidder.key,
                &pda_key,
                bid_amount,
            );
            invoke(&bid_instruction, &[bidder.clone(), pda.clone(), system_program.clone()])?;

            // 更新拍卖状态
            auction_state.highest_bid = bid_amount;
            auction_state.highest_bidder = *bidder.key;
            auction_state.serialize(&mut &mut auction_account.data.borrow_mut()[..])?;
            Ok(())
        }
        AuctionInstruction::CloseAuction => {
            msg!("Closing auction");
            let nft_account = next_account_info(accounts_iter)?;
            let seller = next_account_info(accounts_iter)?;
            let auction_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let highest_bidder = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if auction_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let mut auction_state = AuctionState::try_from_slice(&auction_account.data.borrow())?;
            if !auction_state.active || auction_state.nft_account != *nft_account.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"auction", &auction_state.nft_account.to_bytes()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // 验证拍卖已结束
            let clock = Clock::get()?;
            if clock.unix_timestamp < auction_state.end_time {
                return Err(ProgramError::InvalidAccountData);
            }

            // 转移 NFT 至最高出价者或退回卖家
            let mut nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            if auction_state.highest_bidder != Pubkey::default() {
                nft_state.owner = auction_state.highest_bidder;
                let transfer_instruction = system_instruction::transfer(
                    &pda_key,
                    &auction_state.seller,
                    auction_state.highest_bid,
                );
                invoke_signed(
                    &transfer_instruction,
                    &[pda.clone(), seller.clone(), system_program.clone()],
                    &[&[b"auction", &auction_state.nft_account.to_bytes(), &[bump]]],
                )?;
            } else {
                nft_state.owner = auction_state.seller;
            }
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;

            // 关闭拍卖
            auction_state.active = false;
            auction_state.serialize(&mut &mut auction_account.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}