# Add KernelSVM algorithm in Concrete-ML.
`Machine Learning`

## Overview
Add KernelSVM algorithm in Concrete-ML.

## Description
Add KernelSVM algorithm in Concrete-ML. We expect the submission to contain KernelSVM for both classifications and regressions. One can notably have a look to [SVM SVC](https://scikit-learn.org/stable/modules/generated/sklearn.svm.SVC.html) and [SVM SVR](https://scikit-learn.org/stable/modules/generated/sklearn.svm.SVR.html) scikit-learn documentations for more information about what KernelSVM stands for. One may have a look to how [LinearSVC](https://docs.zama.ai/concrete-ml/developer-guide/api/concrete.ml.sklearn.svm#class-linearsvc) and [LinearSVR](https://docs.zama.ai/concrete-ml/developer-guide/api/concrete.ml.sklearn.svm#class-linearsvr) are supported in Concrete-ML.

In term of deliverables, we expect:
- support of KernelSVM algorithm in Concrete-ML
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
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €5,000

## Related links and references
- [Scikit-Learn SVM SVC](https://scikit-learn.org/stable/modules/generated/sklearn.svm.SVC.html)
- [Scikit-Learn SVM SVR](https://scikit-learn.org/stable/modules/generated/sklearn.svm.SVR.html)
- [Concrete-ML LinearSVC](https://docs.zama.ai/concrete-ml/developer-guide/api/concrete.ml.sklearn.svm#class-linearsvc)
- [Concrete-ML LinearSVR](https://docs.zama.ai/concrete-ml/developer-guide/api/concrete.ml.sklearn.svm#class-linearsvr)
- [Interesting tutorial on Scikit-Learn](https://scikit-learn.org/stable/auto_examples/svm/plot_svm_kernels.html)
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
