# Find an error probability bug in Concrete Numpy

## Category
`Bug hunting` `Cryptography`

## Overview
Find an Error probability bug in Concrete Numpy.

## Description

Each time a function is compiled with Concrete Numpy, Concrete optimizer provides
cryptographic parameters such that it is guaranteeing an error probability for each Table Lookup operations.

Goal of this bounty is to provide a minimal POC that compiles
and runs but with an error probability bigger than the one guaranteed.

To submit the bounty, the candidate needs to provide:
* the concrete-numpy source code corresponding to the error probability bug
* the version of concrete-numpy at the time of submission (only the latest version should be used)
* a report with a reproducible experiment that prove error probability is less than asked.

an example on setting error probability for an input program

```python
import concrete.numpy as cnp
import numpy as np

configuration = cnp.Configuration(
    enable_unsafe_features=True,
    use_insecure_key_cache=True,
    insecure_key_cache_location=".keys",
    p_error=0.01,
)

table = cnp.LookupTable([np.sqrt(x).round().astype(np.int64) for x in range(2 ** 10)])

@cnp.compiler({"x": "encrypted"})
def f(x):
    return table[x]

inputset = [np.random.randint(0, 2 ** 10) for _ in range(100)]
circuit = f.compile(inputset, configuration, verbose=True)

# show that circuit consistently evaluates to an invalid value
# more than expected number of times
```

* change the circuit in a way that it fails more than 1% of the time (should be reproducible over multiple runs)
* you can introduce more table lookups, but the requirements adjust accordingly
for example, if you have 2 tlus back to back, error probablility increases to almost 2% ((1 - (0.99 * 0.99)) * 100) therefore you need to show you got more than 2% errors
* you are also free to change 0.01 value which is used as an example

## Library targeted
[Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)

## Bounty type
[Easy bounty](https://github.com/zama-ai/zama-bounty-program#easy-bounties)

## Reward
€500 to €5,000 depending on the severity of the issue and the quality of the report.

## Related links and references
- [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
- [Developer guide documentation](https://docs.zama.ai/concrete-numpy/developer/)
- [Contributing documentation](https://docs.zama.ai/concrete-numpy/developer/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
