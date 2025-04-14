#![no_std]
#![allow(unexpected_cfgs)]

use instructions::{
    escrow_instructions::EscrowInstruction, make::MakeContext, refund::RefundContext,
    take::TakeContext,
};
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
mod state;

entrypoint!(process_instruction);

// #[cfg(target_os = "solana")]
// pinocchio::nostd_panic_handler!();

pinocchio_pubkey::declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(instruction)? {
        EscrowInstruction::Make => accounts.make(&data.try_into()?),
        EscrowInstruction::Take => accounts.take(),
        EscrowInstruction::Refund => accounts.refund(),
    }
}
