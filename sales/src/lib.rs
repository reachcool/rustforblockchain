use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    program::invoke,
    system_instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};

// NFT 元数据结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct NFTState {
    token_id: u64,        // 唯一标识
    owner: Pubkey,        // 所有者公钥
    metadata_uri: String, // 元数据 URI
}

// 销售状态结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct SaleState {
    nft_account: Pubkey,  // NFT 账户
    seller: Pubkey,       // 卖家公钥
    price: u64,           // 销售价格（Lamports）
    active: bool,         // 销售是否有效
}

// 指令枚举
#[derive(BorshSerialize, BorshDeserialize)]
enum SaleInstruction {
    ListNFT { token_id: u64, price: u64 },
    BuyNFT,
}

// 入口函数
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SaleInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        SaleInstruction::ListNFT { token_id, price } => {
            msg!("Listing NFT with token_id: {} at price: {}", token_id, price);
            let nft_account = next_account_info(accounts_iter)?;
            let seller = next_account_info(accounts_iter)?;
            let sale_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !seller.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if nft_account.owner != program_id || sale_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"sale", &token_id.to_le_bytes()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let rent = Rent::get()?;
            if !rent.is_exempt(sale_account.lamports(), sale_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }

            // 验证 NFT 所有者
            let mut nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            if nft_state.token_id != token_id || nft_state.owner != *seller.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // 初始化销售状态
            let sale_state = SaleState {
                nft_account: *nft_account.key,
                seller: *seller.key,
                price,
                active: true,
            };
            sale_state.serialize(&mut &mut sale_account.data.borrow_mut()[..])?;

            // 转移 NFT 至 PDA 托管
            nft_state.owner = pda_key;
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;
            Ok(())
        }
        SaleInstruction::BuyNFT => {
            msg!("Buying NFT");
            let nft_account = next_account_info(accounts_iter)?;
            let buyer = next_account_info(accounts_iter)?;
            let sale_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let seller = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !buyer.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if nft_account.owner != program_id || sale_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let mut sale_state = SaleState::try_from_slice(&sale_account.data.borrow())?;
            if !sale_state.active || sale_state.nft_account != *nft_account.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"sale", &sale_state.nft_account.to_bytes()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // 验证并转移支付
            if buyer.lamports() < sale_state.price {
                return Err(ProgramError::InsufficientFunds);
            }
            let transfer_instruction = system_instruction::transfer(
                buyer.key,
                seller.key,
                sale_state.price,
            );
            invoke(&transfer_instruction, &[buyer.clone(), seller.clone(), system_program.clone()])?;

            // 更新 NFT 所有权
            let mut nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            nft_state.owner = *buyer.key;
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;

            // 关闭销售
            sale_state.active = false;
            sale_state.serialize(&mut &mut sale_account.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}