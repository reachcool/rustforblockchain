use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    // program::{invoke},
    rent::Rent,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct TokenState {
    mint_authority: Pubkey,
    total_supply: u64,
    decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct TokenAccount {
    owner: Pubkey,
    balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum TokenInstruction {
    Initialize { decimals: u8, initial_supply: u64 },
    Transfer { amount: u64 },
    Burn { amount: u64 },
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TokenInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        TokenInstruction::Initialize { decimals, initial_supply } => {
            msg!("Initializing token with {} decimals, {} supply", decimals, initial_supply);
            let mint_account = next_account_info(accounts_iter)?;
            let mint_authority = next_account_info(accounts_iter)?;
            let token_account = next_account_info(accounts_iter)?;
            // let rent = next_account_info(accounts_iter)?;

            if mint_account.owner != program_id || token_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            let rent_info = Rent::get()?;
            if !rent_info.is_exempt(mint_account.lamports(), mint_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }

            let token_state = TokenState {
                mint_authority: *mint_authority.key,
                total_supply: initial_supply,
                decimals,
            };
            token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

            let token_account_data = TokenAccount {
                owner: *mint_authority.key,
                balance: initial_supply,
            };
            token_account_data.serialize(&mut &mut token_account.data.borrow_mut()[..])?;

            Ok(())
        }
        TokenInstruction::Transfer { amount } => {
            msg!("Transferring {} tokens", amount);
            let source_account = next_account_info(accounts_iter)?;
            let dest_account = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;

            if source_account.owner != program_id || dest_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut source_data = TokenAccount::try_from_slice(&source_account.data.borrow())?;
            let mut dest_data = TokenAccount::try_from_slice(&dest_account.data.borrow())?;

            if !authority.is_signer || source_data.owner != *authority.key {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if source_data.balance < amount {
                return Err(ProgramError::InsufficientFunds);
            }

            source_data.balance -= amount;
            dest_data.balance += amount;

            source_data.serialize(&mut &mut source_account.data.borrow_mut()[..])?;
            dest_data.serialize(&mut &mut dest_account.data.borrow_mut()[..])?;

            Ok(())
        }
        TokenInstruction::Burn { amount } => {
            msg!("Burning {} tokens", amount);
            let account_to_burn = next_account_info(accounts_iter)?;
            let mint_account = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;

            if account_to_burn.owner != program_id || mint_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut token_account_data = TokenAccount::try_from_slice(&account_to_burn.data.borrow())?;
            let mut token_state = TokenState::try_from_slice(&mint_account.data.borrow())?;

            if !authority.is_signer || token_account_data.owner != *authority.key {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if token_account_data.balance < amount {
                return Err(ProgramError::InsufficientFunds);
            }

            token_account_data.balance -= amount;
            token_state.total_supply -= amount;

            token_account_data.serialize(&mut &mut account_to_burn.data.borrow_mut()[..])?;
            token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

            Ok(())
        }
    }
}