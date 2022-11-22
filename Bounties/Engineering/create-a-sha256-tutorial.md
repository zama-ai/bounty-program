# Create a SHA256 tutorial for Concrete Numpy

## Category
`Software`

## Overview
Create a tutorial demonstrating how to develop a SHA256 in concrete Numpy

## Description

Create an FHE program that computes the SHA-256 value over a fixed-length encrypted input.
Turn it into a tutorial for the documentation, highlighting the process of turning a regular
program into its FHE equivalent.

Here is an example of code you could start from:

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

* Input size is fixed to 150 bytes
* Create the example app under `examples/sha256.py`
* Create the tutorial under `docs/tutorial/sha256.md`

## Library targeted
[Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
€7,500

## Related links and references
- [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
- [Developer guide documentation](https://docs.zama.ai/concrete-numpy/developer/)
- [Contributing documentation](https://docs.zama.ai/concrete-numpy/developer/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
