# GLWE Programmable Bootstrapping in TFHE
`Cryptography` `TFHE` `Research`

## Overview
Programmable bootstrapping a GLWE ciphertext, encrypting a polynomial, in TFHE.

## Description
Propose a new technique to do a programmable bootstrapping on a GLWE ciphertext, encrypting a polynomial with 4-bits coefficient messages, in TFHE.
For this challenge, GLWE programmable bootstrapping means that you are able to reduce the noise on all the message coefficients and, at the same time, to evaluate on each of them a look-up table (could be the same for all message coefficients, but preferably a chosen one for each message coefficient).

The solution has to be at least 10x faster than the existing trivial solution, consisting in:
 * extracting all the coefficients as LWE ciphertexts;
 * performing separate programmable bootstrapping on each of them;
 * re-packing them into a single GLWE using packing key-switching.

#### Security and noise
 * The security level of the solution has to be at least 128 bits, strictly under the GLWE problem;
 * The error probability for the chosen parameter set should be at worse $2^{-40}$
 * The noise of the output ciphertext has to be such that at least 3 bits between the message and the noise are empty.

#### Performances and comparison with the state of the art
 * The new programmable bootstrapping must be able to bootstrap correctly a polynomial message with 4-bit coefficients, encrypted as one GLWE ciphertext, and provide in output a GLWE ciphertext encrypted under the same secret key;
 * The public key material (bootstrapping keys, key switching keys, ...) has to remain below 1GB in total;
 * The speed up must be proven by experimental results on the same architecture, **single-threaded**.

Your solution should compare to the following implementation from [TFHE-rs](https://github.com/zama-ai/tfhe-rs):
```
AWS m6i.metal (Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz)
OS: Ubuntu 22.04
```
The parameters for a single bootstrapping on this architechture are:
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
You should compare timings by multiplying the above by the number of messages you can bootstrap at the same time.

#### Potential Approaches

Initial directions could include:


1. Looking at the steps of the trivial solution (listed above) and attempting to improve their performance individually. For example, improving the re-packing technique.
2. Following the approach of B/FV and BGV to perform bootstrapping [1,2]. Their approach is to move the coefficients into the slots of the ciphertext to be able to perform the modular reduction or digit extraction (as would be the case for TFHE) procedure needed when performing decryption homomorphically. With the parameters used in TFHE there are no slots. However, this problem could be overcome by bootstrapping slightly more data than the plaintext bits by simply considering the ciphertext to have an artificial plaintext modulus that is larger and of an appropriate form at the loss of some space for the noise growth.

    A further problem is that the process of performing modular reduction or digit extraction on the slots requires evaluating relatively large degree polynomials. For B/FV and BGV in which the parameters are very large this is feasible. However, for TFHE we typically have much smaller parameters meaning that switching to use much larger parameters (for the outer ciphertexts) to perform the homomorphic decryption would presumably incur a noticeable slow down relative to the amount of data being bootstrapped.

3. The techniques of [3] should be applicable to TFHE, but it is unclear if they give any practical improvement. The main challenge seems to be to find a way to replace the internal product used in FHEW [4] with the external product used in TFHE. An orthogonal route to improving the practical performance of [3] could be to find multiplication algorithms for polynomials better suited for evaluation using homomorphic accumulators.

[1] https://eprint.iacr.org/2014/873</br>
[2] https://eprint.iacr.org/2018/067</br>
[3] https://eprint.iacr.org/2018/532</br>
[4] https://eprint.iacr.org/2014/816</br>

#### Validity of the solutions proposed
A valid submission contains the following:
 * A PDF format (using LaTeX) document, describing in detail the solution proposed;
 * An implementation using TFHE-rs, and instructions to run it;
 * A set of tests aiming to prove the claim on efficiency and instructions to run them.

## Library targeted
[TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
Expert

## Reward
€10,000 to €50,000 depending on performance etc.

## Related links and references
The sources we provide are just indicative (not necessarily the most up to date results and not an exhaustive list of sources):
- TFHE paper: [CGGI20](https://eprint.iacr.org/2018/421);
- [TFHE deep dive](https://www.zama.ai/post/tfhe-deep-dive-part-1)
- [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

### Submission
Apply directly to this bounty by opening an application [here](https://github.com/zama-ai/bounty-program/issues/new?assignees=zaccherinij%2C+aquint-zama&labels=Application&projects=&template=zama-bounty-program--application.md&title=%3Center+Bounty+name%3E).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
