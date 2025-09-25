pragma solidity >=0.8.0;

import '@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol';
import '@uniswap/v3-periphery/contracts/interfaces/INonfungiblePositionManager.sol';
import '@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';

contract LiquidityProvider is IERC721Receiver {
    address public constant WETH = 0xeC7ed8038B76DbcB8F78b189EFf9D7C7373A45BE;
    address public constant TEST_TOKEN = 0xB66dd10F098f62141A536e92f6e8f7f9633893E2;

    uint24 public constant poolFee = 3000;

    INonfungiblePositionManager public immutable nonfungiblePositionManager = INonfungiblePositionManager(0xc744616E5E263B2a0BD344eb3dcD9EDeAFFe8A61);

    event PositionMinted(uint256 indexed tokenId,uint128 liquidity,uint256 amount0,uint256 amount1);

    function onERC721Received(
        address /*operator*/,
        address /*from*/,
        uint256 /*tokenId*/,
        bytes calldata /*data*/
    ) external pure returns (bytes4) {
        revert("TODO");
    }

    function mint()
        external
        returns (
            uint256 tokenId,
            uint128 liquidity,
            uint256 amount0,
            uint256 amount1
        )
    {
        uint256 amount0ToMint = 1000;
        uint256 amount1ToMint = 1000;

        // The contract should be authorized before calling this function to spend the tokens
        TransferHelper.safeTransferFrom(WETH, msg.sender, address(this), amount0ToMint);
        TransferHelper.safeTransferFrom(TEST_TOKEN, msg.sender, address(this), amount1ToMint);

        TransferHelper.safeApprove(WETH, address(nonfungiblePositionManager), amount0ToMint);
        TransferHelper.safeApprove(TEST_TOKEN, address(nonfungiblePositionManager), amount1ToMint);
        // token0 has to be < than token1
        INonfungiblePositionManager.MintParams memory params =
            INonfungiblePositionManager.MintParams({
                token0: TEST_TOKEN,
                token1: WETH,
                fee: poolFee,
                tickLower: -887220,
                tickUpper: 887220,
                amount0Desired: amount0ToMint,
                amount1Desired: amount1ToMint,
                amount0Min: 0,
                amount1Min: 0,
                recipient: address(this),
                deadline: block.timestamp
            });

        (tokenId, liquidity, amount0, amount1) = nonfungiblePositionManager.mint(params);

        emit PositionMinted(tokenId, liquidity, amount0, amount1);
    }
}
