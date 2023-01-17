# Train a deep neural network in FHE that can classify CIFAR10 or ImageNet
`Software` `Concrete-ML`

## Overview
Classification of ImageNet in FHE

## Description

Homomorphically train a Resnet or similar neural network to classify CIFAR10 or Imagenet using TFHE.

Assume that the dataset is owned by an entity C, and is encrypted with a single secret key K only known to C. The training is done by an independant entity S, which outputs a model whose weights are also encrypted by the same key K. The key K is obviously not known by the entity S, but only by C.

This means:
- the training dataset is encrypted with a single secret key K
- the model fits well, i.e., it has an top1 accuracy of at least 0.65 for Imagenet or 0.75 for CIFAR10
- the model will have weights which are encrypted with the secret key
- after the training:
    - to check the model accuracy on the test set: the model weights are decrypted by C with her private key K, and then the inferences can be done over clear test set
    - to use the encrypted model directly on the server S, entity C can encrypt a fresh data with her secret key K and send it to server S, which can run the encrypted inference

In term of machine learning, any technique (in term of quantization, ML model, etc) is acceptable.

In term of deliverables, we expect:
- a script to setup everything, including the key generation
- a script to launch the training
- a script to launch the inferences, once the training is done: possibly, the inferences may be done on the server with the encrypted model, or may be done after final decryption of the trained model with the secret key
- a complete documentation explaining the approach you took to realize this task, and how we can reproduce your results; if your solution use cryptographic constructions or protocols which are not already used in Zama, we also need complete explanations including proof of securities

## Library targeted
[Concrete-ML](https://github.com/zama-ai/concrete-ml)

## Bounty type
[Moonshot bounty](https://github.com/zama-ai/zama-bounty-program#moonshot-bounties)

## Reward
- Up to €50,000 for a CIFAR10 classifier
- Up to €100,000 for an Imagenet classifier

## Related links and references
- [About ImageNet](https://www.image-net.org/index.php)
- [About ResNet](https://en.wikipedia.org/wiki/Residual_neural_network)
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
