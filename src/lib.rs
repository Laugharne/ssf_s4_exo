use solana_program::{
	account_info::AccountInfo,
	entrypoint,
	entrypoint::ProgramResult,
	pubkey::Pubkey,
	msg,
};

// Define the entrypoint function
entrypoint!(process_instruction);

fn process_instruction(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	instruction_data: &[u8],
) -> ProgramResult {
	msg!("Hello, Solana!");
	Ok(())
}

/*
pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
*/