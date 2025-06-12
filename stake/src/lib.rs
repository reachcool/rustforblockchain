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

// 质押状态结构体
#[derive(BorshSerialize, BorshDeserialize)]
struct StakeState {
    user: Pubkey,         // 质押者公钥
    amount: u64,          // 质押 SOL 数量（Lamports）
    start_time: i64,      // 质押开始时间（Unix 时间戳）
    reward_rate: u64,     // 奖励率（每秒奖励 Lamports/质押 Lamports，放大 1e6）
    active: bool,         // 质押是否有效
}

// 指令定义
#[derive(BorshSerialize, BorshDeserialize)]
enum StakeInstruction {
    Stake { amount: u64 },
    Unstake,
}

// 入口函数
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakeInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        StakeInstruction::Stake { amount } => {
            msg!("Staking {} Lamports", amount);
            let user = next_account_info(accounts_iter)?;
            let stake_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let clock = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !user.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if stake_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"stake", user.key.as_ref()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let rent = Rent::get()?;
            if !rent.is_exempt(stake_account.lamports(), stake_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }
            if user.lamports() < amount {
                return Err(ProgramError::InsufficientFunds);
            }

            // 转移 SOL 至 PDA 托管
            let transfer_instruction = system_instruction::transfer(
                user.key,
                pda.key,
                amount,
            );
            invoke(
                &transfer_instruction,
                &[user.clone(), pda.clone(), system_program.clone()],
            )?;

            // 初始化或更新质押状态
            let mut stake_state = if stake_account.data_len() > 0 {
                StakeState::try_from_slice(&stake_account.data.borrow())?
            } else {
                StakeState {
                    user: *user.key,
                    amount: 0,
                    start_time: 0,
                    reward_rate: 1_000, // 示例：每秒 0.001 Lamports/Lamport
                    active: false,
                }
            };
            stake_state.amount = stake_state.amount.checked_add(amount).ok_or(ProgramError::ArithmeticOverflow)?;
            stake_state.start_time = Clock::get()?.unix_timestamp;
            stake_state.active = true;
            stake_state.serialize(&mut &mut stake_account.data.borrow_mut()[..])?;
            Ok(())
        }
        StakeInstruction::Unstake => {
            msg!("Unstaking all Lamports and claiming rewards");
            let user = next_account_info(accounts_iter)?;
            let stake_account = next_account_info(accounts_iter)?;
            let pda = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let clock = next_account_info(accounts_iter)?;

            // 验证权限和账户
            if !user.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            if stake_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let (pda_key, bump) = Pubkey::find_program_address(&[b"stake", user.key.as_ref()], program_id);
            if pda_key != *pda.key {
                return Err(ProgramError::InvalidAccountData);
            }
            let mut stake_state = StakeState::try_from_slice(&stake_account.data.borrow())?;
            if !stake_state.active || stake_state.user != *user.key || stake_state.amount == 0 {
                return Err(ProgramError::InvalidAccountData);
            }

            // 计算奖励
            let amount = stake_state.amount;
            let current_time = Clock::get()?.unix_timestamp;
            let duration = (current_time - stake_state.start_time).max(0) as u64;
            let reward = (duration * amount * stake_state.reward_rate) / 1_000_000;
            let total_amount = amount.checked_add(reward).ok_or(ProgramError::ArithmeticOverflow)?;
            if pda.lamports() < total_amount {
                return Err(ProgramError::InsufficientFunds);
            }

            // 转移 SOL 和奖励
            let transfer_instruction = system_instruction::transfer(
                pda.key,
                user.key,
                total_amount,
            );
            invoke_signed(
                &transfer_instruction,
                &[pda.clone(), user.clone(), system_program.clone()],
                &[&[b"stake", user.key.as_ref(), &[bump]]],
            )?;

            // 更新质押状态
            stake_state.amount = 0;
            stake_state.active = false;
            stake_state.serialize(&mut &mut stake_account.data.borrow_mut()[..])?;
            Ok(())
        }
    }
}