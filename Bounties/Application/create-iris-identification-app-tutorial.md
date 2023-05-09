# Develop an HE-based iris identification application tutorial with Concrete
`Application`

## Overview
Design an iris recognition biometric template protection schemes based on Homomorphic Encryption

## Description

Biometric recognition is becoming a prominent way to authenticate users or verify their identities. As highlighted in the ISO/IEC 24745, it is important to protect biometric information for secrecy, irreversibility, and renewability during storage and transfer.
In this bounty you will need to design an FHE based remote authentication system that protects sensitive Iris information during storage and biometric comparison.
In its paper "Hybrid biometric template protection: Resolving the agony of choice between bloom filters and homomorphic encryption", the research team looked at three different approaches: Bloom filters, homomorphic encryption and hybrid biometric template protection (BTP). The team highlighted the advantages and disadvantages of each approach.

The bounty objective is to:

* use Concrete to implement a single key TFHE-based BTP for an access control system
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
* Create the app `frontends/concrete-python/examples/iris-identification/{client,server}.py`
* Create the tutorial `docs/tutorial/iris-identification.{md,ipynb}`

## Library targeted
* [Concrete](https://github.com/zama-ai/concrete)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €5,000

## Related links and references
* [Concrete documentation](https://docs.zama.ai/concrete)
* [Concrete contribution guide](https://docs.zama.ai/concrete/developer/contributing)
<!-- markdown-link-check-disable-next-line -->
* [Resolving the agony of choice between bloom filters and homomorphic encryption](https://doi.org/10.1049/bme2.12075)
* [UBIRIS.v2](http://iris.di.ubi.pt/ubiris2.html) Use password: UBIRIS2_IEEETPAMI_101109_200966

## Submission
Apply directly to this bounty by opening an application [here](https://github.com/zama-ai/bounty-program/issues/new?assignees=zaccherinij%2C+aquint-zama&labels=Application&projects=&template=zama-bounty-program--application.md&title=%3Center+Bounty+name%3E).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
