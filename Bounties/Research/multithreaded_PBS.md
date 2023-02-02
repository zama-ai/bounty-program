# Implement a multithreaded programmable bootstrap in TFHE
`Cryptography`

## Overview
Accelerate the TFHE programmable bootstrap by executing it multithreaded.

## Description
Find and implement a technique to evaluate a single 4-bit TFHE programmable bootstrap using mutliple threads. The solution has to be at least 10x faster than the existing bootstrap in TFHE-rs.

#### Security and noise
 * The security level of the solution has to be at least 128 bits, strictly under the GLWE problem;
 * The error probability for the chosen parameter set should be at worse $2^{-40}$
 * The noise of the output ciphertext has to be such that at least 3 bits between the message and the noise are empty.

#### Performances and comparison with the state of the art
 * The new programmable bootstrapping must be able to bootstrap correctly a 4-bit message,
 encrypted as one LWE ciphertext, and provide in output a LWE ciphertext encrypted under the same secret key;
 * The public key material (bootstrapping keys, key switching keys, ...) has to remain below 1GB in total;
 * The speed up must be proven by experimental results on the same architecture (**multi-threading** is allowed).

Your solution should compare to the following implementation from [TFHE-rs](https://github.com/zama-ai/tfhe-rs):
```
AWS m6i.metal (Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz)
OS: Ubuntu 22.04
```
The parameters for a single bootstrapping on this architecture are:
```
lwe_dimension: LweDimension(742),
glwe_dimension: GlweDimension(1),
polynomial_size: PolynomialSize(2048),
lwe_modular_std_dev: StandardDev(0.000007069849454709433),
glwe_modular_std_dev: StandardDev(0.00000000000000029403601535432533),
pbs_base_log: DecompositionBaseLog(23),
pbs_level: DecompositionLevelCount(1),
ks_level: DecompositionLevelCount(5),
ks_base_log: DecompositionBaseLog(3),
message_modulus: MessageModulus(4),
```

We use 64-bit integers in the implementation.
The key sizes and timings for a single bootstrapping on this architecture are:
```
Bootstrapping key: 46.38 MB
Bootstrapping key (Fourier): 92.75 MB
Key swiching key: 58.05 MB
Time per PBS single-thread (no avx512): 21.419 ms
Time per PBS single-thread (avx512): 18.396 ms
```

Your timings should be for a single pbs, non-amortized.


#### Validity of the solutions proposed
A valid submission contains the following:
 * A PDF format (using LaTeX) document, describing in detail the solution proposed;
 * An implementation using TFHE-rs, and instructions to run it;
 * A set of tests aiming to prove the claim on efficiency and instructions to run them.

## Library targeted
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
Expert

## Reward
Up to €50,000 depending on performance.

## Related links and references
The sources we provide are just indicative (not necessarily the most up to date results and not an exhaustive list of sources):
- TFHE paper: [CGGI20](https://eprint.iacr.org/2018/421);
- [TFHE deep dive](https://www.zama.ai/post/tfhe-deep-dive-part-1)
- [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
