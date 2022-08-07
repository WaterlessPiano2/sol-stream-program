use std::str::FromStr;

use crate::{
    instruction::StreamInstruction,
    state::{CreateStreamInput, StreamData, WithdrawInput},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = StreamInstruction::unpack(instruction_data)?;

        match instruction {
            StreamInstruction::CreateStream(data) => todo!(),
            StreamInstruction::WithdrawFromStream(data) =>todo!(),
            StreamInstruction::CloseStream => todo!(),
        }
    }
}