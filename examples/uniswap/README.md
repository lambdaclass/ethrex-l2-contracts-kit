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

```shell
yarn install
```

> [!NOTE]
> If you already deployed the contracts once you need to run `rm state.json`

3. Deploy contracts

```shell
NODE_OPTIONS=--openssl-legacy-provider yarn start --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --weth9-address 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be --json-rpc http://localhost:1729 --native-currency-label "ETH" --owner-address 0x0000000000000000000000000000000000000001
```

4. List the deployed contracts

```shell
cat state.json
```

```json
{
  "v3CoreFactoryAddress": "0xBf9646Bf89624f5b1617Cef863E765c82F2ceD5f",
  "multicall2Address": "0x4b8d115d560c7c4988D2B8b84f411406574442Ce",
  "proxyAdminAddress": "0xD0131576FB0e50968e6fe5DEDa1e673297A4EF55",
  "tickLensAddress": "0xdEb1481F062db511358b2497ad1C2eB23DB76225",
  "nftDescriptorLibraryAddressV1_3_0": "0xC3D5369cc2520647E53bB2a68e53ffb1693bf742",
  "nonfungibleTokenPositionDescriptorAddressV1_3_0": "0x39786bd79Fe10D3C72427b4Cdd7923648caE149E",
  "descriptorProxyAddress": "0xC37007BA477Bda14F23a8B69aC0Dfcd798c3f2b5",
  "nonfungibleTokenPositionManagerAddress": "0xc744616E5E263B2a0BD344eb3dcD9EDeAFFe8A61",
  "v3MigratorAddress": "0xB29e716E27Af469b0DA1ba73F0651eE139f0304d",
  "v3StakerAddress": "0x556C04B2B08Dd2f7Cefd01626AC789547649a1E6",
  "quoterV2Address": "0xEE3E6620cb102FcD3b34782E88a4259A192bF1e9",
  "swapRouter02": "0xD16759a138B4800309834377DFcd0a8d68BDb1fB"
}
```

We will use the v3CoreFactoryAddress, nonfungibleTokenPositionManagerAddress and swapRouter02 addresses in this example

### Create a liquidity pool

Next we will create a liquidity pool for the WETH/TEST swap with a 0.3% fee tier:
create the pool with:

```shell
rex send --rpc-url http://localhost:1729 <FACTORY-ADDRESS> --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "createPool(address,address,uint24)" 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 3000
```

You can check the pool exists calling the uniswap factory contract:

```shell
rex call 0xBf9646Bf89624f5b1617Cef863E765c82F2ceD5f "getPool(address,address,uint24)" 0xB66dd10F098f62141A536e92f6e8f7f9633893E2 0xec7ed8038b76dbcb8f78b189eff9d7c7373a45be 3000 --rpc-url http://localhost:1729
```

Will return the address 0xF62dF99F23C52D2721a8B86739bc624162470F00


### Initialize the liquidity pool

initialize the pool with calldata for a 1WETH to 1TEST price:

```shell
rex send --rpc-url http://localhost:1729 0xF62dF99F23C52D2721a8B86739bc624162470F00 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 "initialize(uint160)" 79228162514264337593543950336
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
Contract deployed in tx: 0x1f42faf95e4dce6fd57e34a07a2c2b0bbd83d153574b5a5a509f415c6825501c
Contract address: 0xa0c79e7f98c9914c337d5b010af208b98f23f117
```

But yours could be different. We'll put the address in an ENV_VAR to use in the next commands

```shell
export LIQUIDITY_PROVIDER_ADDRESS=0xa0c79e7f98c9914c337d5b010af208b98f23f117
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
rex call 0xF62dF99F23C52D2721a8B86739bc624162470F00 "liquidity()" --rpc-url http://localhost:1729
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
