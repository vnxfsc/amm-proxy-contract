use arrayref::array_ref;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_unchecked,
    pubkey::Pubkey,
};

pub const ATA_SELECTOR: &[u8; 8] = &[22, 51, 53, 97, 247, 184, 54, 78];

const ATA_PROGRAM: Pubkey = solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub fn process_create_associated_token_account(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [funder_info, associated_token_account_info, spl_token_mint_info, system_program_info, spl_token_program_info] =
        array_ref![accounts, 0, 5];

    let funder_key = *funder_info.key;

    invoke_unchecked(
        &Instruction {
            program_id: ATA_PROGRAM,
            accounts: vec![
                AccountMeta::new(funder_key, true),
                AccountMeta::new(*associated_token_account_info.key, false),
                AccountMeta::new_readonly(funder_key, false),
                AccountMeta::new_readonly(*spl_token_mint_info.key, false),
                AccountMeta::new_readonly(*system_program_info.key, false),
                AccountMeta::new_readonly(*spl_token_program_info.key, false),
            ],
            data: instruction_data.to_vec(),
        },
        accounts,
    )
}
