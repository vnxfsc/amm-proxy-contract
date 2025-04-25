use std::{env, fmt::Error};

use dotenvy::dotenv;
use rand::{distr::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    instruction::{AccountMeta, Instruction},
    pubkey::{self, Pubkey},
    rent::sysvar,
    signer::Signer,
    system_instruction::create_account_with_seed,
    system_program,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::initialize_account;
const RENT_LAMPORTS: u64 = 3000000;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();
    let _ = pump_buy().await;
    let _ = pump_sell().await;
    let _ = raydium_buy().await;
    let _ = raydium_sell().await;
    // raydium_buy().await;
    // let _ = create_lookup_tabl_1().await;
}

pub const GLOBAL_ACCOUNT: Pubkey =
    solana_sdk::pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub const FEE_RECIPIENT: Pubkey =
    solana_sdk::pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV");
const EVENT_AUTHORITY: Pubkey = solana_sdk::pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
pub const PUMP_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

const PROXY_PROGRAM: Pubkey = solana_sdk::pubkey!("AmXoSVCLjsfKrwCUqvkMFXYcDzZ4FeoMYs7SAhGyfMGy");

const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
const AMM_AUTHORITY: Pubkey = solana_sdk::pubkey!("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
const WSOL: Pubkey = solana_sdk::pubkey!("So11111111111111111111111111111111111111112");

pub const PUMP_SELECTOR: &[u8; 8] = &[82, 225, 119, 231, 78, 29, 45, 70];       // 内盘买入鉴别器
pub const PUMP_AMM_SELECTOR: &[u8; 8] = &[129, 59, 179, 195, 110, 135, 61, 2];  // 外盘买入鉴别器
pub const PUMP_SELL_SELECTOR: &[u8; 8] = &[83, 225, 119, 231, 78, 29, 45, 70];  // 内盘卖出鉴别器
pub const PUMP_AMM_SELL_SELECTOR: &[u8; 8] = &[130, 59, 179, 195, 110, 135, 61, 2]; // 外盘卖出鉴别器

pub const ATA_SELECTOR: &[u8; 8] = &[22, 51, 53, 97, 247, 184, 54, 78];
pub const RAYDIUM_BUY_SELECTOR: &[u8; 8] = &[182, 77, 232, 39, 117, 138, 183, 72];
pub const RAYDIUM_SELL_SELECTOR: &[u8; 8] = &[183, 77, 232, 39, 117, 138, 183, 72];

const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";

// 生成判别符
fn generate_discriminant() -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(b"global:pump_buy");
    let hash = hasher.finalize();
    hash[..8].try_into().unwrap()
}

pub fn get_account_seed() -> String {
    let mut rng = rand::rng();

    (0..32)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect::<String>()
}
async fn pump_buy() -> Result<(), Error> {
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_client =
        RpcClient::new_with_commitment("".to_string(), CommitmentConfig::confirmed());

    let token_amount = 351100_u64;
    let max_sol_cost = 11000000_u64;
    let mut data = Vec::with_capacity(24);
    data.extend_from_slice(PUMP_SELECTOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&max_sol_cost.to_le_bytes());

    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let token_mint = solana_sdk::pubkey!("HzTV9ZgLJKGmPnYy2c5Pvganm7A2PsNhWBj3e2sgpump");

    let bonding_curve_address =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, token_mint.as_ref()], &PUMP_PROGRAM_ID);

    let associated_user = get_associated_token_address(&signer.pubkey(), &token_mint);

    let associated_bonding_curve =
        get_associated_token_address(&bonding_curve_address.0, &token_mint);

    let instruction = Instruction::new_with_bytes(
        PROXY_PROGRAM,
        &data,
        vec![
            AccountMeta::new_readonly(GLOBAL_ACCOUNT, false),
            AccountMeta::new(FEE_RECIPIENT, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(bonding_curve_address.0, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::id(), false),
            AccountMeta::new_readonly(EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(PUMP_PROGRAM_ID, false),
        ],
    );
    let blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        })
        .await
        .unwrap();

    let mut ata_data = Vec::with_capacity(9);

    ata_data.extend_from_slice(ATA_SELECTOR);

    ata_data.extend_from_slice(&[0]);

    let ata_instruction = Instruction::new_with_bytes(
        PROXY_PROGRAM,
        &ata_data,
        vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(associated_user, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
    );

    // 创建 Durable Nonce 账户
    let create_nonce_tx = Transaction::new_signed_with_payer(
        &[ata_instruction, instruction],
        Some(&signer.pubkey()),
        &[signer],
        blockhash.0,
    );

    // 发送并确认交易
    let signature = match rpc_client
        .send_transaction_with_config(
            &create_nonce_tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await
    {
        Ok(signature) => signature,
        Err(e) => {
            println!("swap error: {:?}", e);
            return Err(Error);
        }
    };
    println!("signature: {}", signature);
    Ok(())
}

async fn raydium_buy() -> Result<(), Error> {
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_client =
        RpcClient::new_with_commitment("".to_string(), CommitmentConfig::confirmed());
    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let token_amount = 351100_u64;
    let max_sol_cost = 11000000_u64;
    let mut data = Vec::with_capacity(25);

    data.extend_from_slice(RAYDIUM_BUY_SELECTOR);
    data.extend_from_slice(&[9]);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&max_sol_cost.to_le_bytes());

    let output_mint: Pubkey = solana_sdk::pubkey!("DYUjm68jHoQFHMHzuRqomrhRcog9mc4TNrCWHpufpump");

    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let destination =
        spl_associated_token_account::get_associated_token_address(&signer.pubkey(), &output_mint);

    let seed = get_account_seed();
    let program_id = spl_token::id();

    let wsol_pubkey = Pubkey::create_with_seed(&signer.pubkey(), &seed, &program_id).unwrap();
    let mut instructions = Vec::with_capacity(5);

    instructions.extend_from_slice(&[
        create_account_with_seed(
            &signer.pubkey(),
            &wsol_pubkey,
            &signer.pubkey(),
            &seed,
            max_sol_cost + RENT_LAMPORTS,
            165,
            &program_id,
        ),
        initialize_account(&program_id, &wsol_pubkey, &WSOL, &signer.pubkey()).unwrap(),
    ]);

    let amm_pool_id = solana_sdk::pubkey!("B6wsohtrxtsFxpBriMhUcGqcWspMxJcMf7Fzoei8X7d8");
    let amm_coin_vault = solana_sdk::pubkey!("4WTsDp9XMTd7i2kxbgpxMEcp4ypP5VRU3pWuLqxiJxCR");
    let amm_pc_vault = solana_sdk::pubkey!("Ai3GCwGYeGuNq879LKUwyUKeReKV5eLaUpfc7W7DPhf");

    let ix: Instruction = Instruction {
        program_id: PROXY_PROGRAM,
        data,
        accounts: vec![
            AccountMeta::new_readonly(RAYDIUM_PROGRAM_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(amm_pool_id, false),
            AccountMeta::new(AMM_AUTHORITY, false),
            AccountMeta::new(amm_coin_vault, false),
            AccountMeta::new(amm_pc_vault, false),
            AccountMeta::new(wsol_pubkey, false),
            AccountMeta::new(destination, false),
            AccountMeta::new(signer.pubkey(), true),
        ],
    };

    instructions.push(ix);

    let blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        })
        .await
        .unwrap();

    // 创建 Durable Nonce 账户
    let create_nonce_tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&signer.pubkey()),
        &[signer],
        blockhash.0,
    );

    // 发送并确认交易
    let signature = match rpc_client
        .send_transaction_with_config(
            &create_nonce_tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await
    {
        Ok(signature) => signature,
        Err(e) => {
            println!("swap error: {:?}", e);
            return Err(Error);
        }
    };
    println!("signature: {}", signature);
    Ok(())
}

async fn pump_sell() -> Result<(), Error> {
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_client =
        RpcClient::new_with_commitment("".to_string(), CommitmentConfig::confirmed());

    let token_amount = 351100_u64;
    let min_sol_receive = 10000000_u64;
    let mut data = Vec::with_capacity(24);
    data.extend_from_slice(PUMP_SELL_SELECTOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&min_sol_receive.to_le_bytes());

    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let token_mint = solana_sdk::pubkey!("HzTV9ZgLJKGmPnYy2c5Pvganm7A2PsNhWBj3e2sgpump");

    let bonding_curve_address =
        Pubkey::find_program_address(&[BONDING_CURVE_SEED, token_mint.as_ref()], &PUMP_PROGRAM_ID);

    let associated_user = get_associated_token_address(&signer.pubkey(), &token_mint);

    let associated_bonding_curve =
        get_associated_token_address(&bonding_curve_address.0, &token_mint);

    let instruction = Instruction::new_with_bytes(
        PROXY_PROGRAM,
        &data,
        vec![
            AccountMeta::new_readonly(GLOBAL_ACCOUNT, false),
            AccountMeta::new(FEE_RECIPIENT, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(bonding_curve_address.0, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::id(), false),
            AccountMeta::new_readonly(EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(PUMP_PROGRAM_ID, false),
        ],
    );

    let blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        })
        .await
        .unwrap();

    let create_nonce_tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        blockhash.0,
    );

    let signature = match rpc_client
        .send_transaction_with_config(
            &create_nonce_tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await
    {
        Ok(signature) => signature,
        Err(e) => {
            println!("swap error: {:?}", e);
            return Err(Error);
        }
    };
    println!("signature: {}", signature);
    Ok(())
}

async fn raydium_sell() -> Result<(), Error> {
    let private_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_client =
        RpcClient::new_with_commitment("".to_string(), CommitmentConfig::confirmed());
    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let token_amount = 351100_u64;
    let min_sol_receive = 10000000_u64;
    let mut data = Vec::with_capacity(25);

    data.extend_from_slice(RAYDIUM_SELL_SELECTOR);
    data.extend_from_slice(&[9]);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&min_sol_receive.to_le_bytes());

    let input_mint: Pubkey = solana_sdk::pubkey!("DYUjm68jHoQFHMHzuRqomrhRcog9mc4TNrCWHpufpump");

    let signer = solana_sdk::signature::Keypair::from_base58_string(&private_key);

    let source = spl_associated_token_account::get_associated_token_address(&signer.pubkey(), &input_mint);

    let seed = get_account_seed();
    let program_id = spl_token::id();

    let wsol_pubkey = Pubkey::create_with_seed(&signer.pubkey(), &seed, &program_id).unwrap();
    let mut instructions = Vec::with_capacity(5);

    instructions.extend_from_slice(&[
        create_account_with_seed(
            &signer.pubkey(),
            &wsol_pubkey,
            &signer.pubkey(),
            &seed,
            RENT_LAMPORTS,
            165,
            &program_id,
        ),
        initialize_account(&program_id, &wsol_pubkey, &WSOL, &signer.pubkey()).unwrap(),
    ]);

    let amm_pool_id = solana_sdk::pubkey!("B6wsohtrxtsFxpBriMhUcGqcWspMxJcMf7Fzoei8X7d8");
    let amm_coin_vault = solana_sdk::pubkey!("4WTsDp9XMTd7i2kxbgpxMEcp4ypP5VRU3pWuLqxiJxCR");
    let amm_pc_vault = solana_sdk::pubkey!("Ai3GCwGYeGuNq879LKUwyUKeReKV5eLaUpfc7W7DPhf");

    let ix: Instruction = Instruction {
        program_id: PROXY_PROGRAM,
        data,
        accounts: vec![
            AccountMeta::new_readonly(RAYDIUM_PROGRAM_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(amm_pool_id, false),
            AccountMeta::new(AMM_AUTHORITY, false),
            AccountMeta::new(amm_coin_vault, false),
            AccountMeta::new(amm_pc_vault, false),
            AccountMeta::new(source, false),
            AccountMeta::new(wsol_pubkey, false),
            AccountMeta::new(signer.pubkey(), true),
        ],
    };

    instructions.push(ix);

    let blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        })
        .await
        .unwrap();

    let create_nonce_tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&signer.pubkey()),
        &[signer],
        blockhash.0,
    );

    let signature = match rpc_client
        .send_transaction_with_config(
            &create_nonce_tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await
    {
        Ok(signature) => signature,
        Err(e) => {
            println!("swap error: {:?}", e);
            return Err(Error);
        }
    };
    println!("signature: {}", signature);
    Ok(())
}
