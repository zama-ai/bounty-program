# Add bitwise shift operations in Concrete Numpy
`Engineering`

## Overview

Support bitwise shift operations between encrypted integers in Concrete Numpy.

## Description

Add support in Concrete Numpy for the following operations:

* Bitwise Left Shift of encrypted integers from 1 up to 16-bits with other encrypted integers
* Bitwise Right Shift of encrypted integers from 1 up to 16-bits with other encrypted integers

The encrypted shift operator should behave as the regular python shift operators.

Here is an example of the bitwise Left shift operation on 8-bit encrypted values by a 2-bit encrypted value:

```python
import concrete.numpy as cnp
import numpy as np

configuration = cnp.Configuration(
    enable_unsafe_features=True,
    use_insecure_key_cache=True,
    insecure_key_cache_location=".keys",
)

@cnp.compiler({"x": "encrypted", "y": "encrypted"})
def f(x, y):
    return x << y

inputset = [(np.random.randint(0, 2 ** 8), np.random.randint(0, 2 ** 2)) for _ in range(100)]
circuit = f.compile(inputset, configuration, verbose=True)

assert circuit.encrypt_run_decrypt(0b_0010_1100, 2) == 0b_1011_0000
```

Here are some guidance to get you started:

* Ignore candidate operations in fusing candidate search in [concrete/numpy/compilation/utils.py](https://github.com/zama-ai/concrete-numpy/blob/a8bd04af2a85adf80edffc57be02e031241002e6/concrete/numpy/compilation/utils.py#L188-L205)
* Disable generic table lookup asserts for candidate operations in [concrete/numpy/mlir/graph_converter.py](https://github.com/zama-ai/concrete-numpy/blob/a8bd04af2a85adf80edffc57be02e031241002e6/concrete/numpy/mlir/graph_converter.py#L159-L166)
* Create a new converter for candidate operation in [concrete/numpy/mlir/node_converter.py](https://github.com/zama-ai/concrete-numpy/blob/a8bd04af2a85adf80edffc57be02e031241002e6/concrete/numpy/mlir/node_converter.py#L157-L216)
* Use the old implementation if one the inputs is a clear constant to preserve the old semantics
* Implement new MLIR conversion logic (only existing MLIR operations must be used)

We expect your PR to comply with the following:

* Create tests with 100% coverage (hint: make pytest runs without errors)
* Make sure pcc checks are passing (hint: make pcc runs without errors)
* And update documentation

## Library targeted
[Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)

## Bounty type
[Easy bounty](https://github.com/zama-ai/zama-bounty-program#easy-bounties)

## Reward
Up to  €3,500

## Related links and references
- [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
- [Developer guide documentation](https://docs.zama.ai/concrete-numpy/developer/)
- [Contributing documentation](https://docs.zama.ai/concrete-numpy/developer/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
