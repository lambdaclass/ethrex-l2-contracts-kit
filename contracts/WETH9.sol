// SPDX-License-Identifier: MIT
pragma solidity =0.8.29;

import "./interfaces/IERC20L2.sol";

contract WETH9 is IERC20L2 {
    // IERC20L2 implementation
    address public L1_TOKEN = address(0);
    address public constant BRIDGE = address(0xffff);
    constructor(address l1Addr) {
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

    // WETH9 implementation
    string public name = "Wrapped Ether";
    string public symbol = "WETH";
    uint8 public decimals = 18;

    event Deposit(address indexed dst, uint wad);
    event Withdrawal(address indexed src, uint wad);

    mapping(address => uint) public balanceOf;
    mapping(address => mapping(address => uint)) public allowance;

    receive() external payable {
        deposit();
    }
    function deposit() public payable {
        balanceOf[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }
    function withdraw(uint wad) public {
        require(balanceOf[msg.sender] >= wad);
        balanceOf[msg.sender] -= wad;
        payable(msg.sender).transfer(wad);
        emit Withdrawal(msg.sender, wad);
    }

    function totalSupply() public view returns (uint) {
        return address(this).balance;
    }

    function approve(address guy, uint wad) public returns (bool) {
        allowance[msg.sender][guy] = wad;
        emit Approval(msg.sender, guy, wad);
        return true;
    }

    function transfer(address dst, uint wad) public returns (bool) {
        return transferFrom(msg.sender, dst, wad);
    }

    function transferFrom(
        address src,
        address dst,
        uint wad
    ) public returns (bool) {
        require(balanceOf[src] >= wad);

        if (
            src != msg.sender && allowance[src][msg.sender] != type(uint256).max
        ) {
            require(allowance[src][msg.sender] >= wad);
            allowance[src][msg.sender] -= wad;
        }

        balanceOf[src] -= wad;
        balanceOf[dst] += wad;

        emit Transfer(src, dst, wad);

        return true;
    }
}
