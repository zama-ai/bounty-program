# Create a FHE ECDSA signature tutorial with TFHE-rs
`Application`

## Overview
Create a tutorial demonstrating how to generate a ECDSA signature on clear data with an FHE-encrypted private key

## Description

The goal of this bounty is to implement the ECDSA signature algorithm, used on the Ethereum blockchain, in FHE.
It uses the curve `secp256k1`. From an FHE encrypted private key and a clear message, the provided algorithm should
be able to returns an FHE encrypted signature, that once decrypted by the FHE private key is able to be verified
in clear with the EC public Key.

This bounty does not expect EC key generation, or Signature validation function.

We expect your PR to comply with the following:

* Input size is fixed to 32 bytes
* Private Key size is fixed to 32 bytes

Your PR should comply with the following:
* Create the script `tfhe/examples/secp256k1-signature.rs`
* Create the tutorial `tfhe/docs/tutorial/secp256k1-signature.md`

## Library targeted
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €7,500

## Related links and references
- [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs)
- [TFHE-rs contribution guide](https://docs.zama.ai/tfhe-rs/developers/contributing)
- [secp256k1 implementation example](https://github.com/bitcoin-core/secp256k1)
- [secp256k1 unofficial test vectors](https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors)

## Submission
Apply directly to this bounty by opening an application [here](https://github.com/zama-ai/bounty-program/issues/new?assignees=zaccherinij%2C+aquint-zama&labels=Application&projects=&template=zama-bounty-program--application.md&title=%3Center+Bounty+name%3E).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
