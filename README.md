# ethrex-l2-contracts-kit

A collection of smart contracts and deployment tools for ethrex L2 networks, including WETH9 implementation and DeFi integration examples.

## Project Structure

### Core Contracts (`/contracts`)

Contains production-ready smart contracts for L2 deployment:

- **WETH9.sol**: L2 implementation of Wrapped Ether with bridge integration
- **IERC20L2.sol**: Interface extending ERC-20 with L2-specific functionality

### Deployment Scripts (`/scripts`)

Rust-based tooling for contract compilation and deployment:

- **deployer.rs**: Main deployment script with CLI interface
- Built with Cargo, runnable from the scripts directory

**Usage:**
```bash
cd scripts/
cargo run --bin deployer -- --rpc-url <RPC> --private-key <KEY> --l1-token <ADDR> --salt <SALT>
```

See [`docs/deployer.md`](docs/deployer.md) for detailed deployment instructions.

### Examples (`/examples`)

Self-contained example projects demonstrating contract usage:

#### Uniswap V3 Integration (`/examples/uniswap`)

Complete example showing how to:
- Deploy ERC-20 test tokens
- Interact with Uniswap V3 pools
- Perform token swaps
- Provide liquidity


**Running examples:**

Follow instructions in examples/uniswap/README.md

### Documentation (`/docs`)

- **deployer.md**: Comprehensive guide to WETH9 deployment, including:
  - What is WETH and why it matters
  - L1 vs L2 implementations
  - Step-by-step deployment instructions
  - Usage examples and troubleshooting

## 📚 Resources

- [Uniswap V3](https://docs.uniswap.org/contracts/v3/overview)
- [ethrex documentation](https://github.com/lambdaclass/ethrex/tree/main/docs)
- [WETH9 Deployment Guide](docs/deployer.md)
- [Uniswap Integration Example](examples/uniswap/README.md)

## 🔧 Prerequisites

- Rust toolchain
- Solidity compiler (solc)
- Access to an ethrex L2 node
