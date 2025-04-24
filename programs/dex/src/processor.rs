use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions::ata::{process_create_associated_token_account, ATA_SELECTOR};
use crate::instructions::pump::{
    process_pump_amm_buy, process_pump_amm_sell, process_pump_buy, process_pump_sell,
    PUMP_AMM_SELL_SELECTOR, PUMP_AMM_SELECTOR, PUMP_SELL_SELECTOR, PUMP_SELECTOR,
};
use crate::instructions::raydium::{process_raydium_buy, process_raydium_sell, RAYDIUM_BUY_SELECTOR, RAYDIUM_SELL_SELECTOR};
use crate::instructions::slot::{process_expired_slot, EXPIRED_SLOT_SELECTOR};

type SelectorHandler = fn(&[AccountInfo], &[u8]) -> ProgramResult;

const SELECTORS: [(&[u8; 8], SelectorHandler); 8] = [
    (PUMP_SELECTOR, |accounts, rest| {
        process_pump_buy(accounts, rest)
    }),
    (PUMP_AMM_SELECTOR, |accounts, rest: &[u8]| {
        process_pump_amm_buy(accounts, rest)
    }),
    (PUMP_SELL_SELECTOR, |accounts, rest| {
        process_pump_sell(accounts, rest)
    }),
    (PUMP_AMM_SELL_SELECTOR, |accounts, rest| {
        process_pump_amm_sell(accounts, rest)
    }),
    (ATA_SELECTOR, |accounts, rest| {
        process_create_associated_token_account(accounts, rest)
    }),
    (EXPIRED_SLOT_SELECTOR, |_, rest| process_expired_slot(rest)),
    (RAYDIUM_BUY_SELECTOR, |accounts, rest| {
        process_raydium_buy(accounts, rest)
    }),
    (RAYDIUM_SELL_SELECTOR, |accounts, rest| {
        process_raydium_sell(accounts, rest)
    }),
];

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (method, rest) = instruction_data.split_at(8);

    for (selector, handler) in SELECTORS.iter() {
        if method == selector.as_slice() {
            return handler(accounts, rest);
        }
    }

    Err(ProgramError::InvalidInstructionData)
}
