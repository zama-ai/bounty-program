# Develop an HE-based iris identification application tutorial with Concrete Numpy or TFHE-rs
`Application`

## Overview
Design an iris recognition biometric template protection schemes based on Homomorphic Encryption

## Description

Biometric recognition is becoming a prominent way to authenticate users or verify their identities. As highlighted in the ISO/IEC 24745, it is important to protect biometric information for secrecy, irreversibility, and renewability during storage and transfer.
In this bounty you will need to design an FHE based remote authentication system that protects sensitive Iris information during storage and biometric comparison.
In its paper "Hybrid biometric template protection: Resolving the agony of choice between bloom filters and homomorphic encryption", the research team looked at three different approaches: Bloom filters, homomorphic encryption and hybrid biometric template protection (BTP). The team highlighted the advantages and disadvantages of each approach.

The bounty objective is to:

* use Concrete-Numpy or TFHE-rs to implement a single key TFHE-based BTP for an access control system
* all reference templates are stored encrypted in a database on the server

The client:
* collects the iris biometric (format should be the same as UBIRIS.v2)
* extracts the feature vector
* encrypt it
* and send it to the server

Then the server:
* perform the comparison against its database
* send an encrypted matching ID back to client
* a no match encrypted message is returned if no matching template is found

* the system will need to have an Equal Error Rate (EER) of 0.1%
* the UBIRIS.V2 database will be used to compute the error rates

Your PR should comply with the following:
* For Concrete Numpy:
  * Create the app `examples/iris-identification/{client,server}.py`
  * Create the tutorial `docs/tutorial/iris-identification.{md,ipynb}`
* For TFHE-rs:
  * Create the app `tfhe/examples/iris-identification/{client,server}.rs`
  * Create the tutorial `tfhe/docs/tutorial/iris-identification.md`

## Library targeted
* [Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €5,000

## Related links and references
* [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
* [Concrete-Numpy contribution guide](https://docs.zama.ai/concrete-numpy/developer/contributing)
* [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs)
* [TFHE-rs contribution guide](https://docs.zama.ai/tfhe-rs/developers/contributing)
<!-- markdown-link-check-disable-next-line -->
* [Resolving the agony of choice between bloom filters and homomorphic encryption](https://doi.org/10.1049/bme2.12075)
* [UBIRIS.v2](http://iris.di.ubi.pt/ubiris2.html) Use password: UBIRIS2_IEEETPAMI_101109_200966

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
