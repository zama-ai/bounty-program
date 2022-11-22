# Add k-nearest neighbors algorithm in Concrete-ML
`Software` `Concrete-ML` 

## Overview
Add k-nearest neighbors algorithm in Concrete-ML

## Description
The goal of this bounty is to implement the k-nearest neighbors algorithm in Concrete-ML.

What we expect as deliverable:
- support of k-nearest neighbors algorithm in Concrete-ML
- tutorial to explain how to use them, using notably the Virtual Library and FHE computations.

Submission checklist:
- a PR on the public https://github.com/zama-ai/concrete-ml (or a fork of it)
- all tests should be green, no previous test must be removed
- the new functionality must be
    - tested (with pytest)
    - benchmarked (in benchmark/) if it is about a new ML model
    - source-code commented
    - documented (in our docs/ markdowns)

You can also change / add things in Concrete-Numpy, as soon as tests are not broken there. Accuracy in FHE in the tutorial should be close enough to the accuracy of scikit-learn, meaning that the accuracy in FHE is equal to the plaintext reference accuracy with a tolerance of 10%, e.g, an accuracy of 0.8 in scikit-learn and an accuracy larger than 0.72 in FHE.

## Library targeted
[Concrete-ML](https://github.com/zama-ai/concrete-ml)

## Bounty type
[Expert bounty](https://github.com/zama-ai/zama-bounty-program#expert-bounties)

## Reward
€5,000 to €10,000

## Related links and references
- [Scikit-Learn KNeighborsClassifier](https://scikit-learn.org/stable/modules/generated/sklearn.neighbors.KNeighborsClassifier.html)
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
