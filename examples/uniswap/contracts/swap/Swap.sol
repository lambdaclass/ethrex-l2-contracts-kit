pragma solidity >=0.8.0;

import '@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';
import '@swap-router-contracts/contracts/interfaces/IV3SwapRouter.sol';

contract Swap {
    IV3SwapRouter public immutable swapRouter = IV3SwapRouter(0xD16759a138B4800309834377DFcd0a8d68BDb1fB);
    address public constant WETH = 0xeC7ed8038B76DbcB8F78b189EFf9D7C7373A45BE;
    address public constant TEST_TOKEN = 0xB66dd10F098f62141A536e92f6e8f7f9633893E2;

    uint24 public constant poolFee = 3000;

    function swapTestForWeth(uint256 amountIn)
        external
        returns (uint256 amountOut)
    {
        TransferHelper.safeTransferFrom(TEST_TOKEN, msg.sender, address(this), amountIn);
        TransferHelper.safeApprove(TEST_TOKEN, address(swapRouter), amountIn);

        IV3SwapRouter.ExactInputSingleParams memory params = IV3SwapRouter
            .ExactInputSingleParams({
                tokenIn: TEST_TOKEN,
                tokenOut: WETH,
                fee: 3000,
                recipient: msg.sender,
                amountIn: amountIn,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });

        amountOut = swapRouter.exactInputSingle(params);
    }

    
    function swapWethForTest(uint256 amountIn)
        external
        returns (uint256 amountOut)
    {
        TransferHelper.safeTransferFrom(WETH, msg.sender, address(this), amountIn);
        TransferHelper.safeApprove(WETH, address(swapRouter), amountIn);

        IV3SwapRouter.ExactInputSingleParams memory params = IV3SwapRouter
            .ExactInputSingleParams({
                tokenIn: WETH,
                tokenOut: TEST_TOKEN,
                fee: 3000,
                recipient: msg.sender,
                amountIn: amountIn,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });

        amountOut = swapRouter.exactInputSingle(params);
    }
}
