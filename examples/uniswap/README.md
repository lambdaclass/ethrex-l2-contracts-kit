# Swap tokens with Uniswap v3 on ethrex L2

## Prerequisites

- [Yarn](https://yarnpkg.com/)
- [Node](https://nodejs.org/en)
  - We recommend using [NVM](https://www.nvmnode.com/) to manage versions as we need to use v18 and v20.
- [Rex](https://github.com/lambdaclass/rex)

## Steps

### Set Initial Environment Variables

In order for the code to be more concise and understandable we recommend these exports:

RPC URL of the L2 node:
```shell
export RPC_URL=http://localhost:1729
```
> [!NOTE]
> The `RPC_URL` needs to have this name in order for rex to recognize it as default RPC endpoint that it's going to use. Otherwise it must be specified with the `--rpc-url` flag.


Private Key of a Rich Account in L2 (account with high balance):
```shell
export RICH_SK_L2=0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057
```

Contract for deterministic deployments
```shell
export DETERMINISTIC_DEPLOYER=0x4e59b44847b379578588920ca78fbf26c0b4956c
```

### Get WETH on L2

1. Export the L2 WETH address for easier handling

```shell
export WETH_ADDRESS=0x000000000000000000000000000000000000FfFD
```

2. Send some ETH to the contract to mint some WETH

```shell
rex send $WETH_ADDRESS --private-key $RICH_SK_L2 --value 100000000000000000000000
```

This will mint 10 WETH. You can check your new balance with:

```shell
rex call $WETH_ADDRESS "balanceOf(address)" 0x0000bd19F707CA481886244bDd20Bd6B8a81bd3e
```

### Deploy TEST token on L2

1. Deploy TEST contract

We will deploy TEST token, using the deterministic deployer contract that we set up as an env variable at the beginning of the guide. You can get the bytecode of the TEST token from `fixtures/contracts/ERC20/TestToken.bin`. Replace <REST_OF_THE_CODE> with the bytecode in the following command.

```shell
rex send $DETERMINISTIC_DEPLOYER --private-key $RICH_SK_L2 --calldata 0x0000000000000000000000000000000000000000000000000000000000000000<REST_OF_THE_CODE>
```

This should deploy TEST token contract to `0xB66dd10F098f62141A536e92f6e8f7f9633893E2`. You can check this with:

```shell
rex code 0xB66dd10F098f62141A536e92f6e8f7f9633893E2
```

We'll set it as an env variable

```shell
export TEST_TOKEN_ADDRESS=0xB66dd10F098f62141A536e92f6e8f7f9633893E2
```

2. Mint some free TEST tokens to your account

```shell
rex send $TEST_TOKEN_ADDRESS --private-key $RICH_SK_L2 "freeMint()"
```

You can check your new balance with:

```shell
rex call $TEST_TOKEN_ADDRESS "balanceOf(address)" 0x0000bd19F707CA481886244bDd20Bd6B8a81bd3e
```

### Deploy uniswap contracts

1. Clone the deploy-v3 repo

```shell
git clone https://github.com/lambdaclass/deploy-v3
cd deploy-v3
git checkout 84dd40ac
```

2. Install dependencies

> [!NOTE]
> You need to use node 20

```shell
yarn install
```

3. Deploy contracts

> [!NOTE]
> If you have already deployed the contracts before you need to run `rm state.json`

```shell
NODE_OPTIONS=--openssl-legacy-provider yarn start --private-key 0x1bc8b78019f35d4447a774e837d414a3db9e1dea5cfc4e9dc2fc3904969ab51f --weth9-address $WETH_ADDRESS --json-rpc $RPC_URL --native-currency-label "ETH" --owner-address 0x0000000000000000000000000000000000000001
```

4. List the deployed contracts

```shell
cat state.json
```

Ideally they should be the same as this ones, this will happen if the private key used for the deployment hadn't been previously used to send a transaction in the past. If in your case the addresses differ make sure to set the environment variables correctly and in some files of the guide (that will be mentioned) some changes will be required.

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

In this example we will use the v3CoreFactoryAddress, nonfungibleTokenPositionManagerAddress and swapRouter02 addresses.

```shell
export FACTORY_ADDRESS=0xAF66f763079a9026bC7324B5804f28c35f921c8b
```

### Create a liquidity pool

Next we will create a liquidity pool for the WETH/TEST swap with a 0.3% fee tier.

Create the pool with
```shell
rex send $FACTORY_ADDRESS --private-key $RICH_SK_L2 "createPool(address,address,uint24)" $WETH_ADDRESS $TEST_TOKEN_ADDRESS 3000
```

You can check the pool exists by calling the uniswap Factory Contract:

```shell
rex call $FACTORY_ADDRESS "getPool(address,address,uint24)" $TEST_TOKEN_ADDRESS $WETH_ADDRESS 3000
```

This will return the pool address. We'll save it as an environment variable to use it in the following commands.

```shell
export LIQUIDITY_POOL_ADDRESS=0x54af7e84298e1f9e5c4065c14a5848e5f94efec3
```

### Initialize the liquidity pool

Initialize the pool with calldata for a 1WETH to 1TEST price:

```shell
rex send $LIQUIDITY_POOL_ADDRESS --private-key $RICH_SK_L2 "initialize(uint160)" 79228162514264337593543950336
```

> [!NOTE]
> The calldata is `sqrtPriceX96`, which represents the square root of the price ratio of token1 to token0, multiplied by 2^96. In this case `2^96 * √1 = 2^96`. For diving deeper onto this you can read [this blog post from uniswap](https://blog.uniswap.org/uniswap-v3-math-primer).

### Deploy the liquidity provider contract

1. Download contract dependencies

From the root of the repo:

```shell
cd examples/uniswap/contracts/swap
make deps
```

2. Deploy the contract

> [!NOTE]
> If the addresses of the deployed contracts differ from the ones shown in this guide change them in the `LiquidityProvider.sol` file.
> This applies to WETH, TEST_TOKEN and NFT Position Manager.

TODO: Change this for deterministic deployment.

```shell
rex deploy 0 $RICH_SK_L2 \
  --contract-path LiquidityProvider.sol \
  --remappings "@openzeppelin/=deps/openzeppelin-contracts/,@uniswap/=deps/,@openzeppelin/contracts/token/ERC721/IERC721Enumerable.sol=deps/openzeppelin-contracts/contracts/token/ERC721/extensions/IERC721Enumerable.sol,@openzeppelin/contracts/token/ERC721/IERC721Metadata.sol=deps/openzeppelin-contracts/contracts/token/ERC721/extensions/IERC721Metadata.sol"
```
This will output the address of the created contract, yours could be different. We'll put it in an environment variable.

```shell
export LIQUIDITY_PROVIDER_ADDRESS=0x4b8d115d560c7c4988d2b8b84f411406574442ce
```

### Add liquidity to the pool by minting a new position

1. Authorize the Liquidity provider contract to spend your WETH tokens

```shell
rex send $WETH_ADDRESS --private-key $RICH_SK_L2 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 1000000000000000000000000000000
```

2. Authorize the Liquidity provider contract to spend your TEST tokens

```shell
rex send $TEST_TOKEN_ADDRESS --private-key $RICH_SK_L2 "approve(address, uint256)" $LIQUIDITY_PROVIDER_ADDRESS 1000000000000000000000000000000
```

3. Mint a new position

```shell
rex send $LIQUIDITY_PROVIDER_ADDRESS --private-key $RICH_SK_L2 "mint()"
```

4. Check the liquidity of the pool

```shell
rex call $LIQUIDITY_POOL_ADDRESS "liquidity()"
```

### Deploy swap contract

> [!NOTE]
> If the addresses of the deployed contracts differ from the ones shown in this guide change them in the `Swap.sol` file.
> This applies to WETH, TEST_TOKEN and the Swap Router.

```shell
rex deploy 0 $RICH_SK_L2 \
  --contract-path Swap.sol \
  --remappings "@swap-router-contracts/=deps/swap-router-contracts/,@openzeppelin/=deps/openzeppelin-contracts/,@uniswap/v3-periphery/=deps/v3-periphery/,@uniswap/v3-core/=deps/v3-core/"
```

This will output the address of the Swap Contract. We'll put it in an environment variable.
Yours could be different.
TODO: Change this for deterministic deployment.

```shell
export SWAP_CONTRACT_ADDRESS=0xd6a0c08a76a0cde4a1582f33ac25c1e21d9d62d3
```

### Swap TEST for WETH

1. Transfer some TEST and ETH to an empty account

```shell
rex send --private-key $RICH_SK_L2 $TEST_TOKEN_ADDRESS "transfer(address,uint256)" 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF 1000000000000000000
```

```shell
rex send 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF --private-key $RICH_SK_L2 --value 1000000000000000000
```

2. Approve the swap contract to spend TEST tokens

```shell
rex send $TEST_TOKEN_ADDRESS --private-key 0xdd5fcfb45b5702ba0b5c326d0fa29b28dfe4854e3fbd4e104bfae90cefe7732e "approve(address, uint256)" $SWAP_CONTRACT_ADDRESS 10000000000000000000
```

3. Swap TEST for WETH

```shell
rex send $SWAP_CONTRACT_ADDRESS  --private-key 0xdd5fcfb45b5702ba0b5c326d0fa29b28dfe4854e3fbd4e104bfae90cefe7732e "swapTestForWeth(uint256)" 1000000000000000000
```

4. Check WETH balance

```shell
rex call $WETH_ADDRESS "balanceOf(address)" 0x41F31fBf85a69c9F3a1635bBF8F602F6e78F3aDF
```

You should have close to 1WETH minus the 0.3% fee.

# Use the uniswap interface

## Prerequisites

- [Yarn](https://yarnpkg.com/)
- [Node](https://nodejs.org/en)
- [Rex](https://github.com/lambdaclass/rex)
- Follow the instructions from [Swap tokens with Uniswap v3 on ethrex L2](#swap-tokens-with-uniswap-v3-on-ethrex-l2) at least up to [Add liquidity to the pool](#add-liquidity-to-the-pool-by-minting-a-new-position)

## Steps

### Download and build all the dependencies

#### SDK Core

```shell
git clone -b ethrex_support https://github.com/lambdaclass/sdk-core
```

```shell
cd sdk-core
```

Update `src/addresses.ts` from lines 69..77 with the deployed uniswap v3 contracts, also update line 183 with the swapRouter02 address and weth9 address on `src/entities/weth9.ts` if needed

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

Update WETH9 address in `src/util/chains.ts` and `src/providers/token-provider.ts` if neededs

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

Follow the instructions from the readme to deploy the universal router contract to the L2 using the script `DeployEthrexDev.s.sol` you might need to update weth9 and factoryV3 addresses

#### Universal router sdk

```shell
git clone -b ethrex_support https://github.com/lambdaclass/universal-router-sdk
```

```shell
cd universal-router-sdk
```

Update the router constant at `src/utils/constants.ts` with the new router address. Here is needed node <= 18

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

You might need to update the weth9 address in `apps/web/src/constants/tokens.ts`

2. Install the dependencies

```shell
yarn install
```

3. Start the local server

```shell
yarn web start
```

### Perform a swap

1. Perform the swap using the uniswap interface. Look for http://localhost:3000
