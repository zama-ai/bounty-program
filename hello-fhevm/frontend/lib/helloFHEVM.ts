import { ethers } from "ethers";
import { TFHE  } from "fhevmjs";

const rpc = process.env.NEXT_PUBLIC_FHEVM_RPC!;
const contractAddress = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS!;

// ABI – only the functions we’ll call from the UI
const abi = [
  "function setSecret(uint32 _value) external",
  "function increment(euint64 _value) external",
  "function requestDecryption() external",
  "function getEncryptedCount() external view returns (uint256)",
  "function lastDecrypted() external view returns (uint64)",
];

export const getContract = (signer?: ethers.Signer) => {
  const provider = new ethers.JsonRpcProvider(rpc);
  const signerOrProvider = signer ?? provider;
  return new ethers.Contract(contractAddress, abi, signerOrProvider);
};

/** Helper: encrypt a plain uint64 → ciphertext (uint256) */
export const encryptUint64 = (value: number): string => {
  // `TFHE.encryptUint64` returns a hex string prefixed with 0x
  return TFHE.encryptUint64(BigInt(value));
};