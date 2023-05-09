# Create a SHA256 tutorial for Concrete Numpy or TFHE-rs
`Application`

## ✅ Solution 
- This bounty has been solved, see the entire solution here.

## Overview
Create a tutorial demonstrating how to develop a homomorphic SHA256 function

## Description

Create an FHE program that computes the SHA-256 value over an encrypted input.
Turn it into a tutorial for the documentation, highlighting the process of turning a regular
program into its FHE equivalent. Input text could be fixed-length for Concrete Numpy solution,
while it should be any length for TFHE-rs solution.

Here is a python example of code you could start from:

```python
import hashlib

import concrete.numpy as cnp
import numpy as np

text = (
    b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. "
    b"Curabitur bibendum, urna eu bibendum egestas, neque augue eleifend odio, et sagittis viverra."
)
assert len(text) == 150

hasher = hashlib.sha256()
hasher.update(text)

sample_input = list(text)
expected_output = list(hasher.digest())

def sha256(data):
    # TODO: implement this function
    pass

compiler = cnp.Compiler(sha256, {"data": "encrypted"})
circuit = compiler.compile(
    inputset=[
        np.random.randint(0, 2 ** 8, size=(150,))
        for _ in range(100)
    ],
    configuration=cnp.Configuration(
        enable_unsafe_features=True,
        use_insecure_key_cache=True,
        insecure_key_cache_location=".keys",
    ),
    verbose=True,
)
```

We expect your PR to comply with the following:

* For Concrete Numpy:
  * Input size is fixed to 150 bytes
  * Create the app `examples/sha256.py`
  * Create the tutorial `docs/tutorial/sha256.{md,ipynb}`
* For TFHE-rs:
  * Input size is not fixed
  * Create the app `tfhe/examples/sha256.rs`
  * Create the tutorial `tfhe/docs/tutorial/sha256.md`

## Library targeted
* [Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €7,500

## Related links and references
* [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
* [Concrete-Numpy contribution guide](https://docs.zama.ai/concrete-numpy/developer/contributing)
* [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs)
* [TFHE-rs contribution guide](https://docs.zama.ai/tfhe-rs/developers/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
