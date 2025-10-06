# ethrex-l2-contracts-kit

A collection of useful smart contracts for deploying and running an L2 with [ethrex](https://github.com/lambdaclass/ethrex).  
This kit provides ready-to-use building blocks, reference implementations, and utilities to simplify L2 deployments.

## Prerequisites

- Rust toolchain
- Solidity compiler (solc)
- Access to an ethrex L2 node

## Goals
- Provide **reliable references** for common L2 contract needs  
- Keep contracts **minimal, modular, and easy to extend**  
- Make it **easy for devs to bootstrap an L2** with ethrex

## Usage
Clone the repo and use the contracts as starting points for your own L2 deployment:  

```bash
git clone https://github.com/lambdaclass/ethrex-l2-contracts-kit
```

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
cargo run --release -- --rpc-url <RPC> --private-key <KEY> --l1-token <ADDR> --salt <SALT>
```

See [`docs/deployer.md`](docs/deployer.md) for detailed deployment instructions.

### Examples (`/examples`)

Self-contained example projects demonstrating contract usage:

#### Uniswap V3 Integration

Complete example showing how to:
- Deploy ERC-20 test tokens
- Interact with Uniswap V3 pools
- Perform token swaps
- Provide liquidity


Follow instructions in [examples/uniswap](examples/uniswap)

### Documentation (`/docs`)

- **deployer.md**: Comprehensive guide to WETH9 deployment, including:
  - What is WETH and why it matters
  - L1 vs L2 implementations
  - Step-by-step deployment instructions
  - Usage examples and troubleshooting

## Resources

- [ethrex documentation](https://github.com/lambdaclass/ethrex/tree/main/docs)
- [Uniswap V3](https://docs.uniswap.org/contracts/v3/overview)


