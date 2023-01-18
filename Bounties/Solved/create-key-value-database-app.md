# Create a FHE encrypted key-value database example with Concrete Numpy

## Category
`Engineering`

## Overview
Create a FHE key-value database with Concrete-Numpy

## Description

Implement a key-value database where where both keys and values are FHE encrypted.
In particular, the key-value database should be able to:
* add a (key, value) entry if the key has not been used already
* replace a (key, value) entry if the key has already been used. The data space related to the previous value should be reclaimed from the database
* find by key and get only the associated value in database (not a vector of values). The function should return an error flag if no value were found. The time needed to execute the function should be proportional to the size of the database
* the implementation of the delete function is not required

The database should accept 32-bit keys

We expect your PR to comply with the following:

* Changes required in Concrete Numpy code
* Create tests with 100% coverage (hint: make pytest runs without errors)
* Make sure pcc checks are passing (hint: make pcc runs without errors)
* Create the example app under `examples/keyvalue-database/{client,server}.py`
* Create the tutorial under `docs/tutorial/keyvalue-database.md`

## Library targeted
[Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €10,000

## Related links and references
- [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
- [Developer guide documentation](https://docs.zama.ai/concrete-numpy/developer/)
- [Contributing documentation](https://docs.zama.ai/concrete-numpy/developer/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
