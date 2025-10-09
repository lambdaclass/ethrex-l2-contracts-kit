# WETH (Wrapped Ether) in L1 and L2

## Why WETH
WETH (Wrapped Ether) is an ERC-20 token that represents Ether (ETH) at a 1:1 ratio. Each WETH token is backed by 1 ETH held in the WETH contract. WETH was created to solve a fundamental issue: while ETH is the native currency of Ethereum, it doesn't conform to the ERC-20 standard that most tokens and DeFi applications use.

## WETH in Layer 1 (Ethereum Mainnet)
The original WETH contract on Ethereum mainnet is a relatively simple contract that:

1. Accepts ETH deposits and mints an equivalent amount of WETH tokens
2. Burns WETH tokens when users withdraw, returning the equivalent ETH
3. Implements the standard ERC-20 interface

The contract maintains a 1:1 peg through a fully-collateralized model, where every WETH token in circulation is backed by 1 ETH locked in the contract.

## WETH in Layer 2
The WETH contract in L2 is slightly different since we don't want to let bridge WETH from L1→L2 and vice versa.
This is due to a potential collateralization issue:

If WETH were bridged as a regular ERC-20, users on L2 could unwrap it to ETH without any real ETH locked on L1.  
This would create ETH on L2 that isn’t fully backed, breaking the 1:1 guarantee.

To avoid this, any transfer of WETH across layers must first be unwrapped to ETH on the source chain.  
That ETH is then locked and represented appropriately on the destination chain, where it can be wrapped again if needed.  
This flow preserves the full 1:1 backing of WETH while allowing safe movement of value between layers.

## Deployment

Install rex:

```shell
git clone https://github.com/lambdaclass/rex.git
cd rex
make cli
```

Deploy:

```shell
rex deploy 0 <PRIVATE_KEY> \
  --rpc-url <L2_RPC_URL> \
  --contract-path contracts/WETH9.sol \ 
  --remappings "@openzeppelin/contracts=https://github.com/OpenZeppelin/openzeppelin-contracts.git" \
  --salt <SALT> \
  -- "constructor(address)" <L1_WETH_ADDRESS>
```

## Basic usage

Deposit and wrap ETH to WETH:

```shell
rex send 0x2685f8e6309f7b7715a386858161f8f77d6a9591 "deposit()" \
  --rpc-url <L2_RPC_URL> \
  --private-key <PRIVATE_KEY> \
  --value <AMOUNT>
```

Check balance:

```shell
rex call 0x2685f8e6309f7b7715a386858161f8f77d6a9591 "balanceOf(address)" <ADDRESS> \
  --rpc-url <L2_RPC_URL> 
```

Make a withdraw:

```shell
rex send 0x2685f8e6309f7b7715a386858161f8f77d6a9591 "withdraw(uint256)" <AMOUNT> \
  --rpc-url <L2_RPC_URL> \
  --private-key <PRIVATE_KEY>
```

Check new balance:

```shell
rex call 0x2685f8e6309f7b7715a386858161f8f77d6a9591 "balanceOf(address)" <ADDRESS> \
  --rpc-url <L2_RPC_URL> 
```
