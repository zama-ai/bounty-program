# FHE Fixed-Point Arithmetic API (fhe_fixed)

This workspace implements a fixed-point arithmetic API built on top of homomorphic integers provided by [TFHE-rs](https://github.com/zama-ai/tfhe-rs).

## Crate Structure

- `crates/fhe_fixed/`: The `fhe_fixed` library crate.
- `.github/workflows/ci.yml`: Continuous integration workflow.

## Getting Started

1. Install Rust stable and nightly toolchains:
   ```bash
   rustup default stable
   rustup toolchain install nightly
