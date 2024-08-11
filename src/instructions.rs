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

	// In order for accounts to remain active on Solana,
	// they must have a minimum balance to cover the cost
	// of storing data on the blockchain. ("rent").
	// The `Rent::get()` function returns a Rent object that contains information
	// about the cost of storage (the minimum rent required for an account).
	let rent: Rent = Rent::get()?;

	// Calculate the minimum amount of lamports
	// (the smallest unit of SOL currency) required
	// to keep the vault account active on the blockchain.
	let required_lamports: u64 = rent.minimum_balance(
		std::mem::size_of::<Vault>()
	);

	// `required_lamports` contains the amount of lamports required
	// to create and keep the vault account active on Solana.

	// `system_instruction::create_account()` :
	// This function creates an instruction to create a new account.
	// This instruction will be sent to the blockchain to be executed.
	let ix: solana_program::instruction::Instruction = system_instruction::create_account(
		// The public key (address) of the account that will pay for the creation of the new account (user).
		&user.key,
		// The public key of the new account that will be created (vault).
		&vault.key,
		// The amount of lamports to transfer from the user account to the vault account to cover the minimum rent.
		required_lamports,
		// The size in bytes of the vault account, converted to u64.
		std::mem::size_of::<Vault>() as u64,
		// The ID of the Solana program responsible for this new account. This program will have control of the newly created vault account.
		program_id,
	);

	// `ix` contains the account creation instruction ready to be executed.

	// Sends the account creation instruction to the Solana blockchain for execution.
	invoke(
		// The account creation statement we built in the previous step.
		&ix,
		// List of accounts involved in the statement.
		&[
			user.clone          (),   // Account that pays for the account creation,
			vault.clone         (),   // Account that will be created
			system_program.clone(),   // Solana system program that handles account creation.
		]
	)?;

	// If the instruction is successfully executed,
	// the vault account will be created on the Solana blockchain,
	// and the balance required for the rent will be transferred
	// from the user account to the vault account.

	// Access (deserialization) to the data inside the `vault` account
	let mut vault_data: Vault = Vault::try_from_slice(&vault.data.borrow())?;

	vault_data.owner   = *user.key;
	vault_data.balance = 0;

	// Serializes the updated Vault structure and writes that data to
	// the vault account, storing the new values ​​in the blockchain.
	vault_data.serialize(&mut &mut vault.data.borrow_mut()[..])?;
	// `borrow_mut()` is similar to `borrow()`, but this time it allows you
	// to get a mutable reference to the data, meaning you can change it.
	// [&mut ..] takes a mutable slice of all the bytes in vault.data,
	// which allows you to write the serialized data to that account.

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

