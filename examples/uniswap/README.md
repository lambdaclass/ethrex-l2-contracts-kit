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
rex send --rpc-url http://localhost:1729 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --value 100000000000000000000000
```

This will mint 10WETH you can check your new balance with:

```shell
rex call 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be "balanceOf(address)" 0x0000bd19F707CA481886244bDd20Bd6B8a81bd3e --rpc-url http://localhost:1729
```

### Deploy TEST token on L2

1. Deploy TEST contract

We will deploy test token, you can get the bytecode from `contracts/ERC20/ERC20.bin/TestToken.bin`. The bytecode should replace <REST_OF_THE_CODE> in the command

```shell
rex send 0x4e59b44847b379578588920ca78fbf26c0b4956c --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --rpc-url http://localhost:1729 --calldata 0x0000000000000000000000000000000000000000000000000000000000000000<REST_OF_THE_CODE>
```

This should deploy TEST token contract to 0xB66dd10F098f62141A536e92f6e8f7f9633893E2.

2. Mint some free tokens to your account

```shell
rex send 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "freeMint()" --rpc-url http://localhost:1729
```

You can check your new balance with:

```shell
rex call 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 "balanceOf(address)" 0x0000bd19F707CA481886244bDd20Bd6B8a81bd3e --rpc-url http://localhost:1729
```


### Deploy uniswap contracts

1. Clone the deploy-v3 repo

```shell
git clone https://github.com/lambdaclass/deploy-v3
git checkout stable_deployment
git checkout HEAD^
```

2. Install dependencies

> [!NOTE]
> You need to use node 20

```shell
yarn install
```

> [!NOTE]
> If you already deployed the contracts once you need to run `rm state.json`

3. Deploy contracts

```shell
NODE_OPTIONS=--openssl-legacy-provider yarn start --private-key 0x1bc8b78019f35d4447a774e837d414a3db9e1dea5cfc4e9dc2fc3904969ab51f --weth9-address 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --json-rpc http://localhost:1729 --native-currency-label "ETH" --owner-address 0x0000000000000000000000000000000000000001
```

4. List the deployed contracts

```shell
cat state.json
```

```json
{
  "v3CoreFactoryAddress": "0xAF66f763079a9026bC7324B5804f28c35f921c8b",
  "multicall2Address": "0x4040a0A5EA13921Ad00f14A1ab9711e9610AA874",
  "proxyAdminAddress": "0x739019a7FB66e36e533B21263A9cD2cC97E43289",
  "tickLensAddress": "0xeb6596A945014A6Bb072Fe6cd580aF8058a0eAb9",
  "nftDescriptorLibraryAddressV1_3_0": "0x0Ca99b5E83f44D058794386889B9B48802F27E8f",
  "nonfungibleTokenPositionDescriptorAddressV1_3_0": "0xDd034561d4bf0ac9eB67a232432EDf91D33192CE",
  "descriptorProxyAddress": "0xDb9B6e53e4A4A62B6867eedB940D145a869723C0",
  "nonfungibleTokenPositionManagerAddress": "0xC15C9DB90e3523F3Fe45529A05E8F59A16B93486",
  "v3MigratorAddress": "0x4E1f06a3Add264D0B26c2DCBbf22d22715d1EcB8",
  "v3StakerAddress": "0x86b9250aB7518D8b32B29060Ff3157f6de54aaAd",
  "quoterV2Address": "0x2051f1Ae370aB1DA2f66FA88672466170Af23196",
  "swapRouter02": "0x8bFCc50961F2f9c4f2F247eea04293F72088435F"
}
```

We will use the v3CoreFactoryAddress, nonfungibleTokenPositionManagerAddress and swapRouter02 addresses in this example

```shell
export FACTORY_ADDRESS=0xAF66f763079a9026bC7324B5804f28c35f921c8b
```

### Create a liquidity pool

Next we will create a liquidity pool for the WETH/TEST swap with a 0.3% fee tier:
create the pool with:

```shell
rex send --rpc-url http://localhost:1729 $FACTORY-ADDRESS --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "createPool(address,address,uint24)" 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 3000
```

You can check the pool exists calling the uniswap factory contract:

```shell
rex call $FACTORY-ADDRESS "getPool(address,address,uint24)" 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be 3000 --rpc-url http://localhost:1729
```

This will return the pool address We'll put the address in an ENV_VAR to use in the next commands.

```shell
export LIQUIDITY_POOL_ADDRESS=0x3701452a2b3faacfebd24d306ea9da464d607209
```

### Initialize the liquidity pool

initialize the pool with calldata for a 1WETH to 1TEST price:

```shell
rex send --rpc-url http://localhost:1729 $LIQUIDITY_POOL_ADDRESS --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "initialize(uint160)" 79228162514264337593543950336
```

### Deploy the liquidity provider contract

1. Download contract dependencies

From the root of the repo:

```shell
cd examples/uniswap/contracts/swap
make deps
```

2. Check nonfungibleTokenPositionManagerAddress address

On the `LiquidityProvider.sol` file check that the address on line 13 is the same that the output from the [uniswap deployment](#deploy-uniswap-contracts) step

3. Compile the contract

```shell
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

4. Deploy the contract

```shell
rex deploy <CONTRACT-BYTECODE> 0 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --rpc-url http://localhost:1729
```

Copy the bytecode from `contracts/swap/solc_out/LiquidityProvider.bin` rex will return the contract address for example:

```
Contract deployed in tx: 0xb42ecc29fa4c1844db0257687bdce5521556e3e7a0fd25c7fca8a96479d9557e
Contract address: 0x4b8d115d560c7c4988d2b8b84f411406574442ce
```

But yours could be different. We'll put the address in an ENV_VAR to use in the next commands

```shell
export LIQUIDITY_PROVIDER_ADDRESS=0x4b8d115d560c7c4988d2b8b84f411406574442ce
```

### Add liquidity to the pool by minting a new position

1. Authorize the Liquidity provider contract to spend your WETH tokens

```shell
rex send --rpc-url http://localhost:1729 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 1000000000000000000000000000000
```

2. Authorize the Liquidity provider contract to spend your TEST tokens

```shell
rex send --rpc-url http://localhost:1729 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 1000000000000000000000000000000
```

3. Mint a new position

```shell
rex send --rpc-url http://localhost:1729 $LIQUIDITY_PROVIDER_ADDRESS --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "mint()"
```

4. Check the liquidity of the pool

```shell
rex call $LIQUIDITY_POOL_ADDRESS "liquidity()" --rpc-url http://localhost:1729
```

### Deploy swap contract

1. Check swapRouter02 address

On the `Swap.sol` file check that the address on line 7 is the same that the output from the [uniswap deployment](#deploy-uniswap-contracts) step

2. Compile the contract

```shell
solc \
  @swap-router-contracts/=deps/swap-router-contracts/ \
  @openzeppelin/=deps/openzeppelin-contracts/ \
  @uniswap/v3-periphery/=deps/v3-periphery/ \
  @uniswap/v3-core/=deps/v3-core/ \
  --optimize \
  --bin \
  --overwrite \
  --output-dir solc_out \
  Swap.sol
```

3. Deploy the contract

```shell
rex deploy <CONTRACT-BYTECODE> 0 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --rpc-url http://localhost:1729
```

Copy the bytecode from `contracts/swap/solc_out/Swap.bin` rex will return the contract address for example:

```
Contract deployed in tx: 0x1f42faf95e4dce6fd57e34a07a2c2b0bbd83d153574b5a5a509f415c6825501c
Contract address: 0xd6a0c08a76a0cde4a1582f33ac25c1e21d9d62d3
```

But yours could be different. We'll put the address in an ENV_VAR to use in the next commands

```shell
export SWAP_CONTRACT_ADDRESS=0xd6a0c08a76a0cde4a1582f33ac25c1e21d9d62d3
```

### Swap TEST for WETH

1. Transfer some TEST and ETH to an empty account

```shell
rex send --rpc-url http://localhost:1729 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057  0xB66dd10F098f62141A536e92f6e8f7f9633893E2 "transfer(address,uint256)" 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF 1000000000000000000
```

```shell
rex send --rpc-url http://localhost:1729 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --value 10000000
```

2. Approve the swap contract to spend TEST tokens

```shell
rex send --rpc-url http://localhost:1729 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 --private-key 0xdd5fcfb45b5702ba0b5c326d0fa29b28dfe4854e3fbd4e104bfae90cefe7732e "approve(address, uint256)" $SWAP_CONTRACT_ADDRESS 10000000000000000000
```

3. Swap TEST for WETH

```shell
rex send --rpc-url http://localhost:1729 $SWAP_CONTRACT_ADDRESS  --private-key 0xdd5fcfb45b5702ba0b5c326d0fa29b28dfe4854e3fbd4e104bfae90cefe7732e "swapTestForWeth(uint256)" 1000000000000000000
```

4. Check WETH balance

```shell
rex call 0xeC7ed8038B76DbcB8F78b189EFf9D7C7373A45BE "balanceOf(address)" 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF --rpc-url http://localhost:1729
```

You should have close to 1WETH minus the 0.3% fee.

# Use the uniswap interface

## Prerequisites

- [Yarn](https://yarnpkg.com/)
- [Node](https://nodejs.org/en)
- [Rex](https://github.com/lambdaclass/rex)
- Follow the instructions from [Swap tokens with Uniswap v3 on ethrex L2](#swap-tokens-with-uniswap-v3-on-ethrex-l2) up to [Add liquidity to the pool](#add-liquidity-to-the-pool-by-minting-a-new-position)

## Steps

### Download and build all the dependencies

#### SDK Core

```shell
git clone -b ethrex_support https://github.com/lambdaclass/sdk-core
```

```shell
cd sdk-core
```

Update `src/addresses.ts` from lines 69..77 with the deployed uniswap v3 contracts, also update line 183 with the swapRouter02 address

```shell
yarn install
```

```shell
yarn build
```

#### Smart order router

```shell
git clone -b ethrex_support https://github.com/lambdaclass/smart-order-router
```

```shell
cd smart-order-router
```

```shell
npm install
```

```shell
npm run build
```

#### Permit2

```shell
git clone https://github.com/Uniswap/permit2
```

Follow the instructions to deploy the permit2 contract to `0x000000000022D473030F116dDEE9F6B43aC78BA3`

#### Universal router

```shell
git clone -b ethrex_support --recurse-submodules https://github.com/lambdaclass/universal-router
```

Follow the instructions from the readme to deploy the universal router contract to the L2 using the script `DeployEthrexDev.s.sol`

#### Universal router sdk

```shell
git clone -b ethrex_support https://github.com/lambdaclass/universal-router-sdk
```

```shell
cd universal-router-sdk
```

Update the router constant at `src/utils/constants.ts` with the new router address

```shell
yarn install
```

```shell
yarn build
```

#### Interface

### Deploy the uniswap api

1. Clone the repo

```shell
git clone -b ethrex_support https://github.com/lambdaclass/routing-api && cd routing-api
```

2. Install the dependencies

```shell
npm install
```

3. Start the API

```shell
npm run sls:dev
```

### Deploy the interface

1. Clone the repo
```shell
git clone -b ethrex_support https://github.com/lambdaclass/uniswap-interface && cd uniswap-interface
```

2. Install the dependencies

```shell
yarn install
```

3. Start the local server

```shell
yarn web start
```

### Perform a swap

1. Approve the universal router to spend your TEST

```shell
rex send 0x000000000022D473030F116dDEE9F6B43aC78BA3 \
"approve(address,address,uint160,uint48)" \
0xB66dd10F098f62141A536e92f6e8f7f9633893E2 \
<UNIVERSAL-ROUTER-ADDRESS> \
1461501637330902918203684832716283019655932542975 \
281474976710655 \
--private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 \
--rpc-url localhost:1729
```

2. Perform the swap using the uniswap interface
