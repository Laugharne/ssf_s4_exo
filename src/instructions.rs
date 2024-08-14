use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	program::{invoke, invoke_signed},
	program_error::ProgramError,
	pubkey::Pubkey,
	rent::Rent,
	system_instruction, sysvar::{clock::Clock, Sysvar},
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
	pub owner  : Pubkey,
	//pub balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Pda {
	pub signer      : Pubkey,
	pub balance     : u64,
	pub deposit_time: i64,      // Unix timestamp
	pub done        : bool,     // withdraw already done
}


pub const DELAY: i64            = 10;          // seconds
pub const TAG_SSF_PDA: &[u8; 7] = b"SSF_PDA";


pub fn initialize(
	program_id: &Pubkey,
	accounts  : &[AccountInfo]
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
	program_id: &Pubkey,          // The Solana program's public key.
	accounts  : &[AccountInfo],   // The list of accounts involved in this transaction.
	amount    : u64               // The amount of SOL to deposit into the PDA.
) -> ProgramResult {
	let accounts_iter: &mut std::slice::Iter<AccountInfo>  = &mut accounts.iter();

	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let user_pda: &AccountInfo       = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;

	// Derive the PDA using the user's public key
	let (computed_pda, bump_seed) = Pubkey::find_program_address(
		&[
			TAG_SSF_PDA    ,     // A constant seed used to differentiate this PDA.
			user.key.as_ref(),   // The public key of the user.
		],
		program_id  // The program ID for which the address is derived.
	);

	// Verify that the derived PDA matches the provided PDA.
	if user_pda.key != &computed_pda {
		return Err(ProgramError::InvalidAccountData);
	}

	// If the PDA account is empty (new), it needs to be initialized.
	if user_pda.data_is_empty() {
		// Gets the rent exemption amount required to keep the account alive.
		let rent: Rent = Rent::get()?;

		// Calculates the minimum lamports (SOL) needed to create and maintain the account.
		let required_lamports: u64 = rent.minimum_balance(std::mem::size_of::<Pda>());

		// Creates an instruction to create a new account for the PDA.
		let ix: solana_program::instruction::Instruction = system_instruction::create_account(
			&user.key               ,            // The user paying for the creation of the account.
			&computed_pda           ,            // The PDA to be created.
			required_lamports       ,            // The lamports needed for rent exemption.
			std::mem::size_of::<Pda>() as u64,   // Size of the account in bytes.
			program_id              ,            // The program ID for which the account is being created.
		);

		// Never use `signers_seeds` because it uses invoke(),
		// which does not require or allow for signing with a PDA.
		// This is fine for transactions where the PDA does not need to sign.
		// However, in cases where the PDA should sign
		// (like when it’s involved in transfers or when acting as an authority),
		// you must use `invoke_signed()`.
		let signers_seeds: &[&[_]] = &[
			TAG_SSF_PDA    ,     // A constant seed used to differentiate this PDA.
			user.key.as_ref(),   // The public key of the user.
			&[bump_seed]
		];

		// Invokes the instruction to create the PDA account.
		invoke(
			&ix,
			&[
				user.clone          (),   // User's account
				user_pda.clone      (),   // PDA account
				system_program.clone(),   // System program account
			],
		)?;

		// Initialize the PDA's data with default values.
		let mut vault_data: Pda = Pda {
			signer      : *user.key,   // Sets the user as the owner of the PDA.
			balance     : 0,           // Initial balance of the PDA.
			deposit_time: 0,           // Initial deposit timestamp set to 0.
			done        : false,       // Indicates if the PDA is finalized (closed).
		};

		vault_data.serialize(&mut &mut user_pda.data.borrow_mut()[..])?;

	}

	// Serialize the PDA's data into the account's memory.
	let mut vault_data: Pda = Pda::try_from_slice(&user_pda.data.borrow())?;

	if vault_data.done == true {
		return Err(ProgramError::InvalidArgument);
	}

	// Ensure the user is the owner of the vault
	if vault_data.signer != *user.key {
		return Err(ProgramError::IllegalOwner);
	}

	// Update the vault balance
	vault_data.balance = vault_data.balance.checked_add(amount)
		.ok_or(ProgramError::ArithmeticOverflow)?;

	// update timestamp
	// Expressed as Unix time (i.e. seconds since the Unix epoch).
	let clock: Clock        = Clock::get()?;
	vault_data.deposit_time = clock.unix_timestamp as i64;

	vault_data.serialize(&mut &mut user_pda.data.borrow_mut()[..])?;

	// Transfer SOL from user to PDA
	invoke(
		&system_instruction::transfer(&user.key, &user_pda.key, amount),
		&[
			user.clone(),
			user_pda.clone(),
			system_program.clone(),
		],
	)?;

	Ok(())
}


pub fn partial_withdraw(
	program_id: &Pubkey,
	accounts  : &[AccountInfo]
) -> ProgramResult {
	let accounts_iter: &mut std::slice::Iter<AccountInfo>  = &mut accounts.iter();

	let user: &AccountInfo           = next_account_info(accounts_iter)?;
	let vault: &AccountInfo          = next_account_info(accounts_iter)?;
	let system_program: &AccountInfo = next_account_info(accounts_iter)?;

	// Derive the PDA using the user's public key
	let (computed_pda, bump_seed) = Pubkey::find_program_address(
		&[
			TAG_SSF_PDA,
			user.key.as_ref()
		],
		program_id
	);

	// Ensure the derived PDA matches the provided PDA account
	if vault.key != &computed_pda {
		return Err(ProgramError::InvalidAccountData);
	}

	// Deserialize the data from the PDA into a Vault struct
	let mut pda_data: Pda = Pda::try_from_slice(&vault.data.borrow())?;

	// Ensure the user is the owner of the vault
	if pda_data.signer != *user.key {
		return Err(ProgramError::IllegalOwner);
	}

	if pda_data.done == true {
		return Err(ProgramError::Custom(1)); // Custom withdrawal already done
	}

	// Calculate the partial withdrawal amount (1/10th of the vault's balance)
	let withdrawal_amount: u64 = pda_data.balance.checked_div(10)
		.ok_or(ProgramError::InsufficientFunds)?;

	// Ensure the vault has enough balance to withdraw	// Ensure there's enough balance for the withdrawal
	if withdrawal_amount == 0 || pda_data.balance < withdrawal_amount {
		return Err(ProgramError::InsufficientFunds);
	}

	// Deduct the withdrawn amount from the vault's balance
	pda_data.balance = pda_data.balance.checked_sub(withdrawal_amount)
		.ok_or(ProgramError::ArithmeticOverflow)?;

	// update timestamp
	// Expressed as Unix time (i.e. seconds since the Unix epoch).
	let clock: Clock      = Clock::get()?;
	let time_elapsed: i64 = clock.unix_timestamp - pda_data.deposit_time as i64;
	if time_elapsed < DELAY {
		return Err(ProgramError::Custom(0)); // Custom error: Withdrawal too soon
	}

	pda_data.done = true;

	// Serialize the updated vault data back into the PDA account
	pda_data.serialize(&mut &mut vault.data.borrow_mut()[..])?;

	// Transfer the withdrawn SOL from the PDA to the user
	let signers_seeds: &[&[_]] = &[
		TAG_SSF_PDA,            // The tag or label for the PDA
		user.key.as_ref(),      // The user's public key
		&[bump_seed]            // The bump seed to ensure uniqueness
	];

	// Invoke the transfer, using `invoke_signed` to allow the PDA to sign
	invoke_signed(
		&system_instruction::transfer(
			&vault.key,
			&user.key,
			withdrawal_amount
		),
		&[
			vault.clone         (),   // The PDA account
			user.clone          (),   // The user account
			system_program.clone(),   // The system program account
		],
		&[signers_seeds],       // Signers seeds for PDA
	)?;

	Ok(())
}


/*
pub fn transfer_sol_with_cpi(
	accounts: &[AccountInfo],
	amount:   u64
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
*/
