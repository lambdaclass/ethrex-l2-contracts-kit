// SPDX-License-Identifier: MIT
pragma solidity =0.8.29;

import "./interfaces/IERC20L2.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract WETH9 is IERC20L2, ERC20 {
    // IERC20L2 implementation
    address public L1_TOKEN = address(0);
    address public constant BRIDGE = address(0xffff);
    constructor(address l1Addr) ERC20("Wrapped Ether", "WETH") {
        L1_TOKEN = l1Addr;
    }

    modifier onlyBridge() {
        require(msg.sender == BRIDGE, "WETH9: not authorized");
        _;
    }

    function l1Address() external view returns (address) {
        return L1_TOKEN;
    }

    /// We don't allow minting of WETH9 via the bridge, ie cross-chain mints.
    /// Instead users should deposit ETH directly via the WETH9 interface.
    function crosschainMint(address, uint256) external view onlyBridge {
        revert("WETH9: mints are not allowed");
    }

    /// We don't allow burning of WETH9 via the bridge due to not having bridged assets.
    /// Instead users should deposit/withdraw ETH directly via the WETH9 interface.
    function crosschainBurn(address, uint256) external view onlyBridge {
        revert("WETH9: burns are not allowed");
    }

    event Deposit(address indexed dst, uint wad);
    event Withdrawal(address indexed src, uint wad);

    receive() external payable {
        deposit();
    }
    function deposit() public payable {
        _mint(msg.sender, msg.value);
        emit Deposit(msg.sender, msg.value);
    }
    function withdraw(uint wad) public {
        _burn(msg.sender, wad);
        payable(msg.sender).transfer(wad);
        emit Withdrawal(msg.sender, wad);
    }
}
