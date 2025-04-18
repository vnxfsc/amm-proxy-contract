use arrayref::array_ref;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_unchecked,
};

pub const RAYDIUM_BUY_SELECTOR: &[u8; 8] = &[182, 77, 232, 39, 117, 138, 183, 72];
pub const RAYDIUM_SELL_SELECTOR: &[u8; 8] = &[183, 77, 232, 39, 117, 138, 183, 72];

pub fn process_raydium_buy(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [amm_program, token_program, amm_id, amm_authority, amm_coin_vault, amm_pc_vault, user_source_token, user_destination_token, user_source_owner] =
        array_ref![accounts, 0, 9];

    let amm_pool = *amm_id.key;

    invoke_unchecked(
        &Instruction {
            program_id: *amm_program.key,
            accounts: vec![
                AccountMeta::new_readonly(*token_program.key, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new_readonly(*amm_authority.key, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(*amm_coin_vault.key, false),
                AccountMeta::new(*amm_pc_vault.key, false),
                AccountMeta::new_readonly(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new_readonly(amm_pool, false),
                AccountMeta::new(*user_source_token.key, false),
                AccountMeta::new(*user_destination_token.key, false),
                AccountMeta::new_readonly(*user_source_owner.key, true),
            ],
            data: instruction_data.to_vec(),
        },
        accounts,
    )
}

pub fn process_raydium_sell(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [amm_program, token_program, amm_id, amm_authority, amm_coin_vault, amm_pc_vault, user_source_token, user_destination_token, user_source_owner] =
        array_ref![accounts, 0, 9];

    let amm_pool = *amm_id.key;

    invoke_unchecked(
        &Instruction {
            program_id: *amm_program.key,
            accounts: vec![
                AccountMeta::new_readonly(*token_program.key, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new_readonly(*amm_authority.key, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(*amm_coin_vault.key, false),
                AccountMeta::new(*amm_pc_vault.key, false),
                AccountMeta::new_readonly(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new(amm_pool, false),
                AccountMeta::new_readonly(amm_pool, false),
                AccountMeta::new(*user_source_token.key, false),
                AccountMeta::new(*user_destination_token.key, false),
                AccountMeta::new_readonly(*user_source_owner.key, true),
            ],
            data: instruction_data.to_vec(),
        },
        accounts,
    )
}
