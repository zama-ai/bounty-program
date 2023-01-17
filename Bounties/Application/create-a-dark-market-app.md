
## Category
`Application`

## Overview
Create an FHE implementation of the Dark Market algorithm given in https://eprint.iacr.org/2022/923.

## Description

#### Simple Challenge:

In the above mentioned paper the authors implement a simple volume matching Dark Market algorithm using MPC. See Algorithm 1 on page 12, which implements the matching algorithm for a single item (be it instrument, stock, or whatever). You should interepret the secret sharing symbol <x> in this challenge as equivalent to the FHE encryption of the value x under some private key FHE-Enc(k, x).

The algorithm takes as input N sell orders, and M buy orders, where N and M are known to the algorithm. The sell orders are represented by encrypted sell volumes <s_i> for i=1..N, and the buy orders are represented by encrypted buy orders <b_i>, for i=1...M.

The output [line 19] of the algorithm is the opened buy/sell orders which are transacted, those which are not transacted are set to zero during the algorithm.

For your implementation you should sample the values s_i and b_i from the range [1...100]. With N=M=500 the goal is to reach a performance time of under 20 seconds for the execution of lines 1-18 of the algorithm [i.e. not including the decryption step] for TFHE parameters which offer 128-bit security.

The reason for selecting 20 seconds above is that this is time it took the authors of the above paper to implement the algorithm using an MPC system with 100 parties.

#### Complex Challenge:

As a more complex challenge we would like the same algorithm implemend but now with multiple instruments, i.e. multiple different items being bought/sold. The item being sold should be identified by a value in the range [1,...,5000]. The item identifier will be also encrypted, thus each sell order will be of the form (<item_i>, <s_i>).  In real life there will be a skew to the popularity of which stock is being sold/bought the most, e.g. Microsoft trades more (say) than Lidl.

For this more complex problem we expect N=M=500 distinct orders where the instrument identifiers are selected from a Poisson distribution with parameter lambda = 200, but obviously truncate the distribution so the maximum identifier is 5000. The buy/sell volume amounts should be again in the range [1,...,100]. The goal is to obtain an execution of the matching algorithm in under 30 minutes.


#### General Requirements:

We expect your PR to comply with the following:

* You can use any Zama product to implement the algorithm.
* Document any changes you need to make to the underlying products.
* Create tests with 100% coverage (For example: make pytest runs without errors)
* For Concrete Numpy:
  * Create the script `examples/dark-market.py`
  * Create the tutorial `docs/tutorial/dark-market.{md,ipynb}`
* For TFHE-rs:
  * Create the script `tfhe/examples/dark-market.rs`
  * Create the tutorial `tfhe/docs/tutorial/dark-market.md`

There is no need for the code to verify that the encrypted values are in range, one can assume the encryptors are honest.

## Library targeted
* [Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €5,000  [Simple Challenge]
Up to €15,000  [Complex Challenge]

## Related links and references
- [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
- [Concrete-Numpy contribution guide](https://docs.zama.ai/concrete-numpy/developer/contributing)
- [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs)
- [TFHE-rs contribution guide](https://docs.zama.ai/tfhe-rs/developers/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
