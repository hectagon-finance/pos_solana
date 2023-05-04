// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Define our contract and inherit the ERC-20 contract
contract BSCCoin is ERC20 {
    // When the contract is run create a BEP-20 Token
    // The token will be names "BSCCoin"
    // The token will have the symbol "BSCC"
    constructor(uint256 initialSupply) ERC20("Hectagon2", "HTG2") {
        // Create an initial value for the runner of the contract
        _mint(msg.sender, initialSupply * 10 ** decimals());
    }
}
