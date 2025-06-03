use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

// NFT 元数据结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct NFTState {
    token_id: u64,        // 唯一标识
    owner: Pubkey,        // 所有者公钥
    metadata_uri: String, // 元数据 URI
}

// 指令枚举
#[derive(BorshSerialize, BorshDeserialize)]
enum NFTInstruction {
    Mint { token_id: u64, metadata_uri: String },
    Transfer { new_owner: Pubkey },
    GetMetadata,
}

// 入口函数
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NFTInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        NFTInstruction::Mint { token_id, metadata_uri } => {
            msg!("Minting NFT with token_id: {}", token_id);
            let nft_account = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;

            if nft_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            if !authority.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            let rent = Rent::get()?;
            if !rent.is_exempt(nft_account.lamports(), nft_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }

            let nft_state = NFTState {
                token_id,
                owner: *authority.key,
                metadata_uri,
            };
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;
            Ok(())
        }
        NFTInstruction::Transfer { new_owner } => {
            msg!("Transferring NFT to new owner: {}", new_owner);
            let nft_account = next_account_info(accounts_iter)?;
            let current_owner = next_account_info(accounts_iter)?;

            if nft_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            if !current_owner.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let mut nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            if nft_state.owner != *current_owner.key {
                return Err(ProgramError::InvalidAccountData);
            }

            nft_state.owner = new_owner;
            nft_state.serialize(&mut &mut nft_account.data.borrow_mut()[..])?;
            Ok(())
        }
        NFTInstruction::GetMetadata => {
            msg!("Retrieving NFT metadata");
            let nft_account = next_account_info(accounts_iter)?;

            if nft_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            let nft_state = NFTState::try_from_slice(&nft_account.data.borrow())?;
            msg!("Token ID: {}, Owner: {}, Metadata URI: {}", 
                nft_state.token_id, nft_state.owner, nft_state.metadata_uri);
            Ok(())
        }
    }
}