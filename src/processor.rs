use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	account_info::AccountInfo,
	entrypoint::ProgramResult,
	pubkey::Pubkey
};

use crate::instructions::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum NativeVaultInstruction {
	Initialize(),
	Deposit(u64),
	PartialWithdraw(),
	CpiTransfer(u64),
}

pub fn process_instruction(
	program_id: &Pubkey,
	accounts  : &[AccountInfo],
	input     : &[u8],
) -> ProgramResult {
	let instruction: NativeVaultInstruction = NativeVaultInstruction::try_from_slice(input)?;

	match instruction {

		NativeVaultInstruction::Initialize() => initialize(
			program_id,
			accounts,
		),

		NativeVaultInstruction::Deposit(args) => deposit(
			accounts,
			args
		),

		NativeVaultInstruction::PartialWithdraw() => partial_withdraw(
			accounts,
		),

		NativeVaultInstruction::CpiTransfer(args) => transfer_sol_with_cpi(
			accounts,
			args
		),

	}

}
