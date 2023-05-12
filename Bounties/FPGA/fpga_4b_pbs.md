# Implement programmable bootstrapping for 4-bit encrypted integers on FPGA
`FPGA`

## Overview
Implement a low-Latency, high-throughput Programmable Bootstrapping for LWE inputs encrypting 4-bit integers on FPGA.

## Description
To fulfill this bounty, an implementation of a PBS with following parameters should be implemented on a target FPGA card.

The naming conventions used here are from the [blog post](https://www.zama.ai/post/tfhe-deep-dive-part-1) and [tfhe-rs](https://github.com/zama-ai/tfhe-rs).

| Parameter                  | Name in tfhe-rs   | Name in HW | Value |
|----------------------------|-------------------|------------|-------|
| Coefficient Modulus q      |                   | `MOD_Q`    | 2<sup>64</sup> |
| LWE Dimension n            | `lwe_dimension`   | `LWE_K`    | 630   |
| GLWE Dimension k           | `glwe_dimension`  | `GLWE_K`   | 1     |
| Polynomial Size N          | `polynomial_size` | `DEG_N`    | 1024  |
| PBS Decomposition Base Log | `pbs_base_log`    | `PBS_B_W`  | 7     |
| PBS Decomposition Level    | `pbs_level`       | `PBS_L`    | 3     |

Support for the keyswitching is not required.
The decomposition algorithm should follow the algorithm [here](https://eprint.iacr.org/2021/1161.pdf).
The polynomial multiplication should be performed using either:
 - a double-precision floating-point FFT;
 - a fixed-point FFT with an error comparable to a double-precision floating-point FFT;
 - a 64-bit NTT by performing suitable modulo switching operations before the NTT and after the inverse NTT operations.

The radix of the FFT/NTT and the value of the 64-bit modulus for the NTT can be chosen freely.
For the implementation, the parameters should be named as listed in the "Name in HW" column in the table above.

We strongly encourage applicants to implement performance counters relevant to their implemented architecture.

### Target FPGA
The choice of the FPGA is up to you but is limited to a single instance of following cards:
 - Xilinx Alveo U55C
 - Xilinx Alveo U250
 - AWS F1 (Xilinx VU9P)
 - Xilinx Versal VCK5000

The resulting accelerator should be made accessible, either through self-hosting, through an AWS AMI, or through an FPGA cloud solution (e.g. https://www.vmaccel.com/).

### Validation
The reward of the bounty is based on the achieved throughput for the PBS operation on a batch of LWE ciphertexts using a single bootstrapping key, and does not include the initial loading of the bootstrapping key to on/off-chip memory.
More specifically, for the throughput the time is measured from the moment the host receives the first sample-extracted LWE ciphertext result until the last sample-extracted LWE ciphertext results.
For the latency the time is measured from the start of writing the LWE ciphertext/GLWE test polynomial pair to the accelerator until its resulting sample-extracted LWE ciphertext is returned to the host.
- the latency of a single PBS should not exceed 20ms;
- the minimal throughput should be 1,000 PBS/s

An applicant can propose an alternative parameter set if it would help them achieve a higher-throuput or latency.
We can check if a PBS on an LWE ciphertext of a 4-bit encryption is possible with the proposed parameter set, and if so, we will consider its implementation and performance as valid in deciding the reward.

We can provide help with functional validation of the accelerator outputs to applicants at the right stage of the program.

### Submission
A valid submission contains the following:
 - documentation of the architecture, design choices, optimisations, performance measurement methodology, and performance results;
 - RTL code and test benches in a chosen HDL;
 - access to the working accelerator with code and instructions to launch the performance benchmarking.

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
The overall reward is scaled linearly based on throughput as follows:
 - 1,000 PBS/s is rewarded with €10,000
 - 5,000 PBS/s is rewarded with €25,000
 - 10,000 PBS/s is rewarded with €50,000
 - 50,000 PBS/s is rewarded with €75,000
 - 100,000 PBS/s is rewarded with €100,000

## Related links and references
 - [TFHE Deep Dive Blog Posts](https://www.zama.ai/post/tfhe-deep-dive-part-1)
 - [Guide to Fully Homomorphic Encryption over the \[Discretized\] Torus](https://eprint.iacr.org/2021/1402.pdf)
 - [Decomposition Algorithm](https://eprint.iacr.org/2021/1161.pdf)
 - [Rust implementation of TFHE for boolean and small integer arithmetics over encrypted data](https://github.com/zama-ai/tfhe-rs)

## Submission
Apply directly to this bounty by opening an application [here](https://github.com/zama-ai/bounty-program/issues/new?assignees=zaccherinij%2C+aquint-zama&labels=Application&projects=&template=zama-bounty-program--application.md&title=%3Center+Bounty+name%3E).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
