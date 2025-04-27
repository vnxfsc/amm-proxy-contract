# AMM Proxy Contract

本项目是基于 [ChainBuff/amm-proxy-contract](https://github.com/ChainBuff/amm-proxy-contract) 进行功能完善的 Solana 区块链 Swap 合约。

## 项目来源

本项目基于 [ChainBuff/amm-proxy-contract](https://github.com/ChainBuff/amm-proxy-contract) 进行开发，主要完善了以下功能：

1. **添加卖出功能**：
   - 为 Raydium 添加了卖出操作
   - 为 Pump 添加了普通卖出和 AMM 卖出操作
   - 完善了相关的测试用例

2. **功能特性**：
   - 支持多个主流 DEX 的交易操作
     - Raydium 买入和卖出
     - Pump 买入和卖出（包括普通交易和 AMM 交易）
   - 提供关联代币账户（ATA）管理
   - 支持时间槽管理
   - 高性能优化
     - 最高级别编译优化
     - 完整的链接时优化（LTO）
     - 单一代码生成单元

## 最近更新

### 2024年4月更新 - 修复Pump协议指令鉴别器不匹配问题

**问题描述**:
在使用Pump协议普通交易(内盘)进行卖出操作时，交易失败并返回错误：
```
"Instruction #1 Failed - custom program error: 101 | Fallback functions are not supported"
```

**原因分析**:
内盘和外盘的sell指令鉴别器配置错误。原先假设内盘sell鉴别器是`[103, 6, 61, 18, 1, 218, 235, 234]`，但经过交易分析发现正确的鉴别器是`[51, 230, 133, 164, 1, 127, 131, 173]`，与外盘sell鉴别器相同。

**修复方案**:
1. 更新`PUMPFUN_SELL_SELECTOR`为正确的内盘sell鉴别器: `[51, 230, 133, 164, 1, 127, 131, 173]`
2. 明确定义`PUMPAMM_SELL_SELECTOR`为外盘sell鉴别器: `[51, 230, 133, 164, 1, 127, 131, 173]`（与内盘相同）
3. 同时定义`PUMPAMM_BUY_SELECTOR`以提高代码清晰度: `[102, 6, 61, 18, 1, 218, 235, 234]`

## 支持的 DEX

1. **Raydium**
   - 支持买入和卖出操作
   - 通过 `process_raydium_buy` 和 `process_raydium_sell` 函数处理

2. **Pump**
   - 支持四种交易操作：
     - 普通买入 (`process_pump_buy`)
     - AMM 买入 (`process_pump_amm_buy`)
     - 普通卖出 (`process_pump_sell`)
     - AMM 卖出 (`process_pump_amm_sell`)

## 项目结构

```
amm-proxy-contract/
├── programs/                    # 智能合约代码目录
│   └── dex/                     # DEX 代理合约
│       ├── src/                 # 源代码目录
│       │   ├── lib.rs          # 合约入口文件
│       │   ├── processor.rs    # 指令处理器
│       │   └── instructions/   # 指令模块目录
│       │       ├── raydium.rs  # Raydium 相关操作
│       │       ├── pump.rs     # Pump 相关操作
│       │       ├── ata.rs      # 关联代币账户管理
│       │       └── slot.rs     # 时间槽管理
│       └── Cargo.toml          # 合约项目配置文件
├── tests/                       # 测试代码目录
│   ├── src/                    # Rust 测试源码
│   └── Cargo.toml             # 测试项目配置文件
├── Cargo.toml                   # 工作空间配置文件
├── Cargo.lock                   # 依赖锁定文件
└── README.md                    # 项目说明文档
```

### 主要组件

1. **指令处理器 (processor.rs)**
   - 处理所有传入的指令
   - 根据指令选择器路由到相应的处理函数

2. **指令模块 (instructions/)**
   - `raydium.rs`: Raydium DEX 相关操作
   - `pump.rs`: Pump DEX 相关操作
   - `ata.rs`: 关联代币账户管理
   - `slot.rs`: 时间槽管理


## 开发环境要求

- Rust 1.65.0 或更高版本
- Solana CLI 工具
- Anchor 框架（可选）
- Node.js 16+

## 安装依赖

### Rust 依赖
```bash
cargo build
```

## 构建和测试

1. 克隆项目
```bash
git clone https://github.com/vnxfsc/amm-proxy-contract.git
cd amm-proxy-contract
```

2. 构建项目
```bash
cargo build
```

3. 运行测试
```bash
cd tests
cargo run
```

## 部署

1. 构建发布版本
```bash
cargo build-bpf
```

2. 部署到 Solana 网络
```bash
solana program deploy target/deploy/amm_proxy_contract.so
```

## 部署信息

- 代理合约程序ID: `AmXoSVCLjsfKrwCUqvkMFXYcDzZ4FeoMYs7SAhGyfMGy`
- Pump程序ID: `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- PumpAMM程序ID: `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`
- Raydium程序ID: `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8`

## 使用示例

### Raydium 交易

```rust
// 买入示例
let buy_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: RAYDIUM_BUY_SELECTOR.to_vec(),
};

// 卖出示例
let sell_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: RAYDIUM_SELL_SELECTOR.to_vec(),
};
```

### Pump 交易

```rust
// 普通买入示例
let buy_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: PUMP_SELECTOR.to_vec(),
};

// 普通卖出示例
let sell_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: PUMP_SELL_SELECTOR.to_vec(),
};

// AMM 买入示例
let amm_buy_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: PUMP_AMM_SELECTOR.to_vec(),
};

// AMM 卖出示例
let amm_sell_instruction = Instruction {
    program_id: PROGRAM_ID,
    accounts: vec![
        // 账户列表
    ],
    data: PUMP_AMM_SELL_SELECTOR.to_vec(),
};
```

## 注意事项

- 使用前请确保账户有足够的代币和 SOL 用于交易
- 建议在测试网进行充分测试后再部署到主网
- 交易时请注意滑点和手续费

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

MIT License

## 联系方式
- 电 报：[小P](https://t.me/caobizhiwang)
- 交流群：[Buff社区](https://t.me/chainbuff)

## 环境变量

在运行测试之前，需要设置以下环境变量：

```bash
# .env 文件
PRIVATE_KEY=你的私钥
```

## 测试

### Rust 测试
```bash
cd tests
cargo run
``` 
