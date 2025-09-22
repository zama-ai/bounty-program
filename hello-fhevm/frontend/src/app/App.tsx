import { useState } from "react";
import { ethers } from "ethers";
import { FHEVM } from "fhevmjs";

const provider = new ethers.providers.JsonRpcProvider(
  process.env.NEXT_PUBLIC_FHEVM_RPC
);
const signer = provider.getSigner();

async function encryptAndSend(value: number) {
  const ct = await FHEVM.encryptUint8(value);
  const contract = new ethers.Contract(
    process.env.NEXT_PUBLIC_CONTRACT_ADDRESS!,
    [
      "function add(bytes calldata _ct) external",
    ],
    signer
  );
  const tx = await contract.add(ct);
  await tx.wait();
}

export default function App() {
  const [input, setInput] = useState(0);
  const [result, setResult] = useState<string>("");

  const handleSubmit = async () => {
    await encryptAndSend(input);
    // fetch decrypted result
    const contract = new ethers.Contract(
      process.env.NEXT_PUBLIC_CONTRACT_ADDRESS!,
      ["function get() view returns (bytes)"],
      provider
    );
    const ct = await contract.get();
    const clear = await FHEVM.decryptUint8(ct);
    setResult(`Counter = ${clear}`);
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h1>Hello FHEVM</h1>
      <input
        type="number"
        value={input}
        onChange={(e) => setInput(Number(e.target.value))}
      />
      <button onClick={handleSubmit}>Encrypt → Call → Decrypt</button>
      <p>{result}</p>
    </div>
  );
}