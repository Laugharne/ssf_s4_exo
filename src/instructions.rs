use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	program::invoke,
	pubkey::Pubkey,
    rent::Rent,
	system_instruction, sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub last_withdrawal: u64,  // Unix timestamp
}

pub fn initialize(
    program_id: &Pubkey,
	accounts: &[AccountInfo]
) -> ProgramResult {

	let accounts_iter: &mut std::slice::Iter<AccountInfo> = &mut accounts.iter();

	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let vault: &AccountInfo          = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;

	let rent: Rent             = Rent::get()?;
	let required_lamports: u64 = rent.minimum_balance(
		std::mem::size_of::<Vault>()
	);
    let ix = system_instruction::create_account(
        &user.key,
        &vault.key,
        required_lamports,
        std::mem::size_of::<Vault>() as u64,
        program_id,
    );

	Ok(())
}

pub fn deposit(
	accounts: &[AccountInfo],
	amount: u64
) -> ProgramResult {
	let accounts_iter  = &mut accounts.iter();

	Ok(())
}

pub fn partial_withdraw(
	accounts: &[AccountInfo]
) -> ProgramResult {
	let accounts_iter  = &mut accounts.iter();

	Ok(())
}

pub fn transfer_sol_with_cpi(
	accounts: &[AccountInfo],
	amount: u64
) -> ProgramResult {
	let accounts_iter  = &mut accounts.iter();
	let payer          = next_account_info(accounts_iter)?;
	let recipient      = next_account_info(accounts_iter)?;
	let system_program = next_account_info(accounts_iter)?;

	invoke(
		&system_instruction::transfer(
			payer.key,
			recipient.key,
			amount
		),
		&[
			payer.clone(),
			recipient.clone(),
			system_program.clone()
		],
	)?;

	Ok(())
}

