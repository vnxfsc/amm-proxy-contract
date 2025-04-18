use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_unchecked,
    pubkey,
    pubkey::Pubkey,
};

const PUMPFUN_BUY_SELECTOR: &[u8; 8] = &[102, 6, 61, 18, 1, 218, 235, 234];

pub const PUMP_SELECTOR: &[u8; 8] = &[82, 225, 119, 231, 78, 29, 45, 70];
pub const PUMP_AMM_SELECTOR: &[u8; 8] = &[129, 59, 179, 195, 110, 135, 61, 2];

const PUMP_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
const PUMP_AMM_PROGRAM_ID: Pubkey = pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

const ARG_LEN: usize = 24;

fn to_account_metas(accounts: &[AccountInfo]) -> Vec<AccountMeta> {
    let mut metas = Vec::with_capacity(accounts.len());
    metas.append(
        &mut accounts
            .iter()
            .map(|acc| match acc.is_writable {
                false => AccountMeta::new_readonly(*acc.key, acc.is_signer),
                true => AccountMeta::new(*acc.key, acc.is_signer),
            })
            .collect(),
    );
    metas
}

pub fn process_pump_buy(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let mut data = Vec::with_capacity(ARG_LEN);
    data.extend_from_slice(PUMPFUN_BUY_SELECTOR);
    data.extend_from_slice(instruction_data);

    invoke_unchecked(
        &Instruction {
            program_id: PUMP_PROGRAM,
            accounts: to_account_metas(accounts),
            data,
        },
        accounts,
    )
}

pub fn process_pump_amm_buy(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let mut data = Vec::with_capacity(ARG_LEN);
    data.extend_from_slice(PUMPFUN_BUY_SELECTOR);
    data.extend_from_slice(instruction_data);

    invoke_unchecked(
        &Instruction {
            program_id: PUMP_AMM_PROGRAM_ID,
            accounts: to_account_metas(accounts),
            data,
        },
        accounts,
    )
}
