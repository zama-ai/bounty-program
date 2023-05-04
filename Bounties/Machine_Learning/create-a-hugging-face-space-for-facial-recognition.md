# Create a new hugging face space using Concrete-ML for facial recognition
`Machine Learning`

## Overview
Create a new Hugging Face Space demonstrating how to do facial recognition in FHE, using Concrete-ML.

## Description
Create a new Hugging Face Space demonstrating how to do facial recognition in FHE, using Concrete-ML, on the Labeled Faces in the Wild dataset. The metric should be accuracy and should be at least 90% in FHE.

We’ve started with our Sentiment Analysis (https://huggingface.co/spaces/zama-fhe/encrypted_sentiment_analysis) and we would like to go further, this time with a facial recognition: the application would take two encrypted facial images, and return an encrypted boolean, telling whether the faces in the two images are the same.

For Hugging Face-spaces bounties, we expect candidates to provide directly in HF some new spaces, which show something. If the candidate wants, the space can be left private to Zama and the candidate, until it is validated as the winner, or directly made public whatever the result.

You can use any model architecture that you find fit for this task. Concrete-ML supports VGG-style neural networks [built with Brevitas]( https://docs.zama.ai/concrete-ml/deep-learning/torch_support), with arbitrary number of layers, out of the box. One interesting NN architecture would be [DeepFace](https://www.cs.toronto.edu/~ranzato/publications/taigman_cvpr14.pdf), excluding the alignment and frontalization parts. Usually training is done using the triplet or siamese network procedure.

## Library targeted
[Concrete-ML](https://github.com/zama-ai/concrete-ml)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
€3,000 to €5,000

## Related links and references
- [Labeled Faces in the Wild dataset](https://www.kaggle.com/datasets/jessicali9530/lfw-dataset)
- [DeepFace](https://www.cs.toronto.edu/~ranzato/publications/taigman_cvpr14.pdf)
- [Our Sentiment Analysis Hugging Face Space](https://huggingface.co/spaces/zama-fhe/encrypted_sentiment_analysis)
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
