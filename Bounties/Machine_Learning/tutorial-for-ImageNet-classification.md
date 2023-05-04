# Run inference in FHE for ImageNet
`Machine Learning`

## Overview
Running inference in FHE for an ImageNet classifier

## Description
The goal of this bounty is to train an ImageNet classifier in the clear, and convert it to FHE using Concrete-ML to be able to perform encrypted inference.

You can use any techniques you want (in term of quantization, ML model, etc). We expect a top1 accuracy >= 0.5 or a top5 accuracy >= 0.8.

In term of deliverables, we expect:
- code and scripts to train (in plaintext) and run the classifier (in FHE)
- a tutorial that explains how it's done

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
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
The reward depends on execution time per inference (without amortization, i.e. one input at a time), computed on a single AWS c6a.metal machine.

Inference time:
- 60-90 min: €5,000
- 30-60 min: €10,000
- 10-30 min: €20,000
- 3-10 min: €30,000
- 0-3 min: €50,000

## Related links and references
- [About ImageNet](https://www.image-net.org/index.php)
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
