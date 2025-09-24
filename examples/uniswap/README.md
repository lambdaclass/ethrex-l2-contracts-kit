# Swap tokens with Uniswap v3 on ethrex L2

## Prerequisites

- [Yarn](https://yarnpkg.com/)
- [Node](https://nodejs.org/en)
- [Rex](https://github.com/lambdaclass/rex)

## Steps

### Deploy WETH9 contract on L2

1. Deploy the contract
On the root of the repo

```shell
cd scripts
```

```shell
cargo run --bin deployer -- --rpc-url http://localhost:1729 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --l1-token 0x0000000000000000000000000000000000000000 --salt 0x0000000000000000000000000000000000000000000000000000000000000000
```

This should deploy WETH9 to the address 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be

2. Send some eth to the contract to mint some WETH

```shell
rex send --rpc-url http://localhost:1729 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --value 10000000000000000000
```

This will mint 10WETH you can check your new balance with

```shell
rex call 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be "balanceOf(address)" 0x0000bd19F707CA481886244bDd20Bd6B8a81bd3e --rpc-url http://localhost:1729
```

### Deploy TEST token on L2

We will deploy test token, you can get the bytecode from `conctracts/ERC20/ERC20.bin/TestToken.bin`. The bytecode should replace <REST_OF_THE_CODE> in the command

```shell
rex send 0x4e59b44847b379578588920ca78fbf26c0b4956c --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --rpc-url http://localhost:1729 --calldata 0x0000000000000000000000000000000000000000000000000000000000000000<REST_OF_THE_CODE>
```

This should deploy TEST token contract to 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 and mint some tokens for the deployer account.


### Deploy uniswap contracts

1. Clone the deploy-v3 repo

```shell
git clone https://github.com/lambdaclass/deploy-v3
git checkout stable_deployment
git checkout HEAD^
```

2. Install dependencies

```shell
yarn install
```

> [!NOTE]
> If you already deployed the contracts once you need to run `rm state.json`

3. Deploy contracts

```shell
NODE_OPTIONS=--openssl-legacy-provider yarn start --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --weth9-address 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --json-rpc http://localhost:1729 --native-currency-label "ETH" --owner-address 0x0000000000000000000000000000000000000000
```

### Create a liquidity pool

Next we will create a liquidity pool for the WETH/TEST swap with a 0.3% fee tier:
create the pool with:

```shell
rex send --rpc-url http://localhost:1729 0x17435ccE3d1B4fA2e5f8A08eD921D57C6762A180 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "createPool(address,address,uint24)" 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 3000
```

You can check the pool exists calling the uniswap factory contract


### Initialize the liquidity pool

initialize the pool with calldata for a 1WETH to 1TEST price:

```
rex send --rpc-url http://localhost:1729 0x1617C0299a3b5cA2B849610B53Fc4b7d6De519ab --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "initialize(uint160)" 79228162514264337593543950336
```

### Deploy the liquidity provider contract

1. Download contract dependencies

```shell
cd contracts/
make deps
```

2. Compile the contract

```
solc \
  --optimize \
  --bin \
  --overwrite \
  --output-dir solc_out \
  @openzeppelin/=deps/openzeppelin-contracts/ \
  @uniswap/=deps/ \
  @openzeppelin/contracts/token/ERC721/IERC721Enumerable.sol=deps/openzeppelin-contracts/contracts/token/ERC721/extensions/IERC721Enumerable.sol \
  @openzeppelin/contracts/token/ERC721/IERC721Metadata.sol=deps/openzeppelin-contracts/contracts/token/ERC721/extensions/IERC721Metadata.sol \
  LiquidityProvider.sol
```

3. Deploy the contract

```
rex deploy <CONTRACT-BYTECODE> 0 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057
```

Copy the bytecode from `contracts/swap/solc_out/LiquidityProvider.bin` rex will return the contract address for example:

```
Contract deployed in tx: 0x3a97260be63648830cfbcd735e56535e84b45e96e1d636e0a76c94a03c1946ac
Contract address: 0x4fb49ddf5b2172c94c1f260fcebde0dbb41ce672
```

But yours could be different. We'll put the address in an ENV_VAR to use in the next commands

```shell
export LIQUIDITY_PROVIDER_ADDRESS=0x4fb49ddf5b2172c94c1f260fcebde0dbb41ce672
```

### Add liquidity to the pool by minting a new position

1. Authorize the Liquidity provider contract to spend your WETH tokens

```shell
rex send --rpc-url http://localhost:1729 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 10000000000000000000
```

2. Authorize the Liquidity provider contract to spend your TEST tokens

```shell
rex send --rpc-url http://localhost:1729 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 10000000000000000000
```

3. Mint a new position

```shell
rex send --rpc-url http://localhost:1729 $LIQUIDITY_PROVIDER_ADDRESS --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "mint()"
```

4. Check the liquidity of the pool

```shell
rex call 0x1617C0299a3b5cA2B849610B53Fc4b7d6De519ab "liquidity()" --rpc-url http://localhost:1729
```
