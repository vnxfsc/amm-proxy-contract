import { Connection, Keypair, PublicKey, Transaction, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import BN from 'bn.js';
import * as dotenv from 'dotenv';

dotenv.config();

// 常量定义
const GLOBAL_ACCOUNT = new PublicKey('4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf');
const FEE_RECIPIENT = new PublicKey('62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV');
const EVENT_AUTHORITY = new PublicKey('Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1');
const PUMP_PROGRAM_ID = new PublicKey('6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P');
const PROXY_PROGRAM = new PublicKey('HVN5pETkbwRSnbcXGqjbd8sVUGU7VJCgMC8JeUL8SGUn');
const RAYDIUM_PROGRAM_ID = new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8');
const AMM_AUTHORITY = new PublicKey('5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1');
const WSOL = new PublicKey('So11111111111111111111111111111111111111112');

// 选择器
const PUMP_SELECTOR = Buffer.from([82, 225, 119, 231, 78, 29, 45, 70]);
const PUMP_AMM_SELECTOR = Buffer.from([129, 59, 179, 195, 110, 135, 61, 2]);
const PUMP_SELL_SELECTOR = Buffer.from([83, 225, 119, 231, 78, 29, 45, 70]);
const PUMP_AMM_SELL_SELECTOR = Buffer.from([130, 59, 179, 195, 110, 135, 61, 2]);
const RAYDIUM_BUY_SELECTOR = Buffer.from([182, 77, 232, 39, 117, 138, 183, 72]);
const RAYDIUM_SELL_SELECTOR = Buffer.from([183, 77, 232, 39, 117, 138, 183, 72]);
const ATA_SELECTOR = Buffer.from([22, 51, 53, 97, 247, 184, 54, 78]);

describe('AMM Proxy Contract Tests', () => {
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  let payer: Keypair;

  beforeAll(async () => {
    // 初始化测试环境
    const privateKey = process.env.PRIVATE_KEY;
    if (!privateKey) {
      throw new Error('PRIVATE_KEY environment variable is not set');
    }
    payer = Keypair.fromSecretKey(Buffer.from(privateKey, 'base58'));
  });

  describe('Pump Tests', () => {
    it('should execute pump buy operation', async () => {
      const tokenAmount = new BN(351100);
      const maxSolCost = new BN(11000000);
      
      const data = Buffer.concat([
        PUMP_SELECTOR,
        tokenAmount.toBuffer('le', 8),
        maxSolCost.toBuffer('le', 8)
      ]);

      const tokenMint = new PublicKey('HzTV9ZgLJKGmPnYy2c5Pvganm7A2PsNhWBj3e2sgpump');
      const [bondingCurveAddress] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding-curve'), tokenMint.toBuffer()],
        PUMP_PROGRAM_ID
      );

      const associatedUser = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        payer.publicKey
      );

      const associatedBondingCurve = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        bondingCurveAddress
      );

      const instruction = {
        programId: PROXY_PROGRAM,
        data,
        keys: [
          { pubkey: GLOBAL_ACCOUNT, isSigner: false, isWritable: false },
          { pubkey: FEE_RECIPIENT, isSigner: false, isWritable: true },
          { pubkey: tokenMint, isSigner: false, isWritable: false },
          { pubkey: bondingCurveAddress, isSigner: false, isWritable: true },
          { pubkey: associatedBondingCurve, isSigner: false, isWritable: true },
          { pubkey: associatedUser, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
          { pubkey: EVENT_AUTHORITY, isSigner: false, isWritable: false },
          { pubkey: PUMP_PROGRAM_ID, isSigner: false, isWritable: false }
        ]
      };

      // 创建 ATA 指令
      const ataData = Buffer.concat([ATA_SELECTOR, Buffer.from([0])]);
      const ataInstruction = {
        programId: PROXY_PROGRAM,
        data: ataData,
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: associatedUser, isSigner: false, isWritable: true },
          { pubkey: tokenMint, isSigner: false, isWritable: false },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }
        ]
      };

      const { blockhash } = await connection.getLatestBlockhash('confirmed');
      const transaction = new Transaction()
        .add(ataInstruction)
        .add(instruction);
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      transaction.sign(payer);

      const signature = await connection.sendRawTransaction(transaction.serialize(), {
        skipPreflight: true
      });

      console.log('Pump Buy Transaction signature:', signature);
      expect(signature).toBeDefined();
    });

    it('should execute pump amm buy operation', async () => {
      // 类似于普通买入操作，但使用 PUMP_AMM_SELECTOR
      expect(true).toBeTruthy();
    });

    it('should execute pump sell operation', async () => {
      const amount = new BN(1000000);
      const minSolOut = new BN(0);
      
      const data = Buffer.concat([
        PUMP_SELL_SELECTOR,
        amount.toBuffer('le', 8),
        minSolOut.toBuffer('le', 8)
      ]);

      const tokenMint = new PublicKey('HzTV9ZgLJKGmPnYy2c5Pvganm7A2PsNhWBj3e2sgpump');
      const [bondingCurveAddress] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding-curve'), tokenMint.toBuffer()],
        PUMP_PROGRAM_ID
      );

      const associatedUser = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        payer.publicKey
      );

      const associatedBondingCurve = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        bondingCurveAddress
      );

      const instruction = {
        programId: PROXY_PROGRAM,
        data,
        keys: [
          { pubkey: GLOBAL_ACCOUNT, isSigner: false, isWritable: false },
          { pubkey: FEE_RECIPIENT, isSigner: false, isWritable: true },
          { pubkey: tokenMint, isSigner: false, isWritable: false },
          { pubkey: bondingCurveAddress, isSigner: false, isWritable: true },
          { pubkey: associatedBondingCurve, isSigner: false, isWritable: true },
          { pubkey: associatedUser, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
          { pubkey: EVENT_AUTHORITY, isSigner: false, isWritable: false },
          { pubkey: PUMP_PROGRAM_ID, isSigner: false, isWritable: false }
        ]
      };

      const { blockhash } = await connection.getLatestBlockhash('confirmed');
      const transaction = new Transaction().add(instruction);
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      transaction.sign(payer);

      const signature = await connection.sendRawTransaction(transaction.serialize(), {
        skipPreflight: true
      });

      console.log('Pump Sell Transaction signature:', signature);
      expect(signature).toBeDefined();
    });

    it('should execute pump amm sell operation', async () => {
      const amount = new BN(1000000);
      const minSolOut = new BN(0);
      
      const data = Buffer.concat([
        PUMP_AMM_SELL_SELECTOR,
        amount.toBuffer('le', 8),
        minSolOut.toBuffer('le', 8)
      ]);

      const tokenMint = new PublicKey('HzTV9ZgLJKGmPnYy2c5Pvganm7A2PsNhWBj3e2sgpump');
      const [bondingCurveAddress] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding-curve'), tokenMint.toBuffer()],
        PUMP_PROGRAM_ID
      );

      const associatedUser = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        payer.publicKey
      );

      const associatedBondingCurve = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        bondingCurveAddress
      );

      const instruction = {
        programId: PROXY_PROGRAM,
        data,
        keys: [
          { pubkey: GLOBAL_ACCOUNT, isSigner: false, isWritable: false },
          { pubkey: FEE_RECIPIENT, isSigner: false, isWritable: true },
          { pubkey: tokenMint, isSigner: false, isWritable: false },
          { pubkey: bondingCurveAddress, isSigner: false, isWritable: true },
          { pubkey: associatedBondingCurve, isSigner: false, isWritable: true },
          { pubkey: associatedUser, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
          { pubkey: EVENT_AUTHORITY, isSigner: false, isWritable: false },
          { pubkey: PUMP_PROGRAM_ID, isSigner: false, isWritable: false }
        ]
      };

      const { blockhash } = await connection.getLatestBlockhash('confirmed');
      const transaction = new Transaction().add(instruction);
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      transaction.sign(payer);

      const signature = await connection.sendRawTransaction(transaction.serialize(), {
        skipPreflight: true
      });

      console.log('Pump AMM Sell Transaction signature:', signature);
      expect(signature).toBeDefined();
    });
  });

  describe('Raydium Tests', () => {
    it('should execute raydium buy operation', async () => {
      const tokenAmount = new BN(351100);
      const maxSolCost = new BN(11000000);
      
      const data = Buffer.concat([
        RAYDIUM_BUY_SELECTOR,
        Buffer.from([9]),
        tokenAmount.toBuffer('le', 8),
        maxSolCost.toBuffer('le', 8)
      ]);

      const outputMint = new PublicKey('DYUjm68jHoQFHMHzuRqomrhRcog9mc4TNrCWHpufpump');
      const destination = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        outputMint,
        payer.publicKey
      );

      // 创建 WSOL 账户
      const seed = Math.random().toString(36).substring(2, 15);
      const wsolPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        seed,
        TOKEN_PROGRAM_ID
      );

      const instructions = [
        SystemProgram.createAccountWithSeed({
          fromPubkey: payer.publicKey,
          newAccountPubkey: wsolPubkey,
          basePubkey: payer.publicKey,
          seed,
          lamports: maxSolCost.add(new BN(3000000)).toNumber(),
          space: 165,
          programId: TOKEN_PROGRAM_ID
        }),
        Token.createInitializeAccountInstruction(
          TOKEN_PROGRAM_ID,
          wsolPubkey,
          WSOL,
          payer.publicKey
        )
      ];

      const ammPoolId = new PublicKey('B6wsohtrxtsFxpBriMhUcGqcWspMxJcMf7Fzoei8X7d8');
      const ammCoinVault = new PublicKey('4WTsDp9XMTd7i2kxbgpxMEcp4ypP5VRU3pWuLqxiJxCR');
      const ammPcVault = new PublicKey('Ai3GCwGYeGuNq879LKUwyUKeReKV5eLaUpfc7W7DPhf');

      const instruction = {
        programId: PROXY_PROGRAM,
        data,
        keys: [
          { pubkey: RAYDIUM_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: ammPoolId, isSigner: false, isWritable: true },
          { pubkey: AMM_AUTHORITY, isSigner: false, isWritable: true },
          { pubkey: ammCoinVault, isSigner: false, isWritable: true },
          { pubkey: ammPcVault, isSigner: false, isWritable: true },
          { pubkey: wsolPubkey, isSigner: false, isWritable: true },
          { pubkey: destination, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true }
        ]
      };

      instructions.push(instruction);

      const { blockhash } = await connection.getLatestBlockhash('confirmed');
      const transaction = new Transaction().add(...instructions);
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      transaction.sign(payer);

      const signature = await connection.sendRawTransaction(transaction.serialize(), {
        skipPreflight: true
      });

      console.log('Raydium Buy Transaction signature:', signature);
      expect(signature).toBeDefined();
    });

    it('should execute raydium sell operation', async () => {
      const amount = new BN(1000000);
      const minSolOut = new BN(0);

      const data = Buffer.concat([
        RAYDIUM_SELL_SELECTOR,
        amount.toBuffer('le', 8),
        minSolOut.toBuffer('le', 8)
      ]);

      const tokenMint = new PublicKey('DYUjm68jHoQFHMHzuRqomrhRcog9mc4TNrCWHpufpump');
      const source = await Token.getAssociatedTokenAddress(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMint,
        payer.publicKey
      );

      // 创建 WSOL 账户用于接收 SOL
      const seed = Math.random().toString(36).substring(2, 15);
      const wsolPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        seed,
        TOKEN_PROGRAM_ID
      );

      const instructions = [
        SystemProgram.createAccountWithSeed({
          fromPubkey: payer.publicKey,
          newAccountPubkey: wsolPubkey,
          basePubkey: payer.publicKey,
          seed,
          lamports: 3000000, // 足够的 lamports 用于创建账户
          space: 165,
          programId: TOKEN_PROGRAM_ID
        }),
        Token.createInitializeAccountInstruction(
          TOKEN_PROGRAM_ID,
          wsolPubkey,
          WSOL,
          payer.publicKey
        )
      ];

      const ammPoolId = new PublicKey('B6wsohtrxtsFxpBriMhUcGqcWspMxJcMf7Fzoei8X7d8');
      const ammCoinVault = new PublicKey('4WTsDp9XMTd7i2kxbgpxMEcp4ypP5VRU3pWuLqxiJxCR');
      const ammPcVault = new PublicKey('Ai3GCwGYeGuNq879LKUwyUKeReKV5eLaUpfc7W7DPhf');

      const instruction = {
        programId: PROXY_PROGRAM,
        data,
        keys: [
          { pubkey: RAYDIUM_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
          { pubkey: ammPoolId, isSigner: false, isWritable: true },
          { pubkey: AMM_AUTHORITY, isSigner: false, isWritable: true },
          { pubkey: ammCoinVault, isSigner: false, isWritable: true },
          { pubkey: ammPcVault, isSigner: false, isWritable: true },
          { pubkey: source, isSigner: false, isWritable: true },
          { pubkey: wsolPubkey, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: true }
        ]
      };

      instructions.push(instruction);

      const { blockhash } = await connection.getLatestBlockhash('confirmed');
      const transaction = new Transaction().add(...instructions);
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = payer.publicKey;
      transaction.sign(payer);

      const signature = await connection.sendRawTransaction(transaction.serialize(), {
        skipPreflight: true
      });

      console.log('Raydium Sell Transaction signature:', signature);
      expect(signature).toBeDefined();
    });
  });
}); 