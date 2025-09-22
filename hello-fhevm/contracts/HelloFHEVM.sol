// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/*
 *  Hello FHEVM – a minimal confidential contract
 *  • Stores an encrypted counter (euint64) on‑chain.
 *  • Anyone can increment the counter with an encrypted value.
 *  • Provides helpers to read the raw ciphertext or the decrypted count.
 *
 *  Uses Zama’s TFHE library from the @zama/fhevm package.
 */
import "fhevm/lib/TFHE.sol";

contract HelloFHEVM {
    // Encrypted 64‑bit unsigned integer stored as ciphertext
    euint64 private encryptedCount;

    // Initialise the counter to an encrypted zero
    constructor() {
        encryptedCount = TFHE.encryptUint64(0);
    }

    // Increment the encrypted counter with an encrypted value
    function increment(euint64 _value) external {
        encryptedCount = TFHE.add(encryptedCount, _value);
    }

    // Return the raw ciphertext (still encrypted)
    function getEncryptedCount() external view returns (euint64) {
        return encryptedCount;
    }

    // Decrypt the counter for the caller (via fhevmjs off‑chain)
    function getDecryptedCount() external view returns (uint64) {
        return TFHE.decryptUint64(encryptedCount);
    }
}