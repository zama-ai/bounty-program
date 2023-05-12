# Find an error probability bug in Concrete
`Engineering`

## Overview
Find an Error probability bug in Concrete.

## Description

Each time a function is compiled with Concrete, Concrete optimizer provides
cryptographic parameters such that it is guaranteeing an error probability for each Table Lookup operations.

Goal of this bounty is to provide a minimal POC that compiles
and runs but with an error probability bigger than the one guaranteed.

To submit the bounty, the candidate needs to provide:
* the concrete-python source code corresponding to the error probability bug
* the version of concrete-python at the time of submission (only the latest version should be used)
* a report with a reproducible experiment that prove error probability is less than asked.

an example on setting error probability for an input program

```python
from concrete import fhe
import numpy as np

configuration = fhe.Configuration(
    enable_unsafe_features=True,
    use_insecure_key_cache=True,
    insecure_key_cache_location=".keys",
    p_error=0.01,
)

table = fhe.LookupTable([np.sqrt(x).round().astype(np.int64) for x in range(2 ** 10)])

@fhe.compiler({"x": "encrypted"})
def f(x):
    return table[x]

inputset = [np.random.randint(0, 2 ** 10) for _ in range(100)]
circuit = f.compile(inputset, configuration, verbose=True)

# show that circuit consistently evaluates to an invalid value
# more than expected number of times
```

* change the circuit in a way that it fails more than 1% of the time (should be reproducible over multiple runs)
* you can introduce more table lookups, but the requirements adjust accordingly
for example, if you have 2 tlus back to back, error probability increases to almost 2% ((1 - (0.99 * 0.99)) * 100) therefore you need to show you got more than 2% errors
* you are also free to change 0.01 value which is used as an example

## Library targeted
[Concrete](https://github.com/zama-ai/concrete)

## Bounty type
[Easy bounty](https://github.com/zama-ai/bounty-program#easy-bounties)

## Reward
€500 to €5,000 depending on the severity of the issue and the quality of the report.

## Related links and references
- [Concrete documentation](https://docs.zama.ai/concrete)
- [Contributing documentation](https://docs.zama.ai/concrete/developer/contributing)

## Submission
Apply directly to this bounty by opening an application [here](https://github.com/zama-ai/bounty-program/issues/new?assignees=zaccherinij%2C+aquint-zama&labels=Application&projects=&template=zama-bounty-program--application.md&title=%3Center+Bounty+name%3E).


## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
