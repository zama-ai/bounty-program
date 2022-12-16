# Create an application "Play Chess" in FHE with Concrete-ML
`Production` `Concrete-ML`

## Overview
Create an application that plays Chess against an AI oponent. Your moves should be encrypted with FHE so that the AI doesn't see them but can still run its algorithm on them.

## Description
Create a machine-learning-based version of a Chess player which can be executed in FHE, i.e., where the computer does not see your unencrypted moves (but can still beat you). Different attempts to play Chess with Deep Learning have been proposed recently, see some references that we picked from the Net. We would like to try to replicate some of the results, but adding encryption on top of this: on the player side, the board would be in clear; then, when she plays her move, she encrypts the new position and sends it to the server, which then runs the machine-learning model inference over encrypted data, to predict a new (encrypted) move to apply. Finally, the player decrypts this move and apply it on the position, and reiterate the process until the game is over.

The AI should be at least rating of 1500 ELO on Lichess.

What we expect:
- a client / server application that enables playing chess via the command line (or whatever you think is good!)
- a trained ML model and corresponding Concrete-ML inference implementation
- a tutorial explaining how you built it

## Library targeted
[Concrete-ML](https://github.com/zama-ai/concrete-ml)

## Bounty type
[Easy bounty](https://github.com/zama-ai/zama-bounty-program#easy-bounties)

## Reward
€2,500 - €5,000

## Related links and references
- [Concrete-ML documentation](https://docs.zama.ai/concrete-ml)
- [Train Your Own Chess AI](https://towardsdatascience.com/train-your-own-chess-ai-66b9ca8d71e4)
- [Creating A Chess AI using Deep Learning](https://towardsdatascience.com/creating-a-chess-ai-using-deep-learning-d5278ea7dcf)
- [Developer guide documentation](https://docs.zama.ai/concrete-ml)
- [Contributing documentation](https://docs.zama.ai/concrete-ml/developer-guide/contributing)
- [Lichess](https://lichess.org)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
