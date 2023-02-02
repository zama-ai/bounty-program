# Create a homomorphic regex engine and write a tutorial about it

## Overview
Create a homomorphic regular expression engine that operate on encrypted bytes of an ASCII string and write a tutorial about it.

## Description

Your implementation should support following regular expression features:

* Contains matching
  * `/abc/` should only match with strings containing `abc` (e.g., `abc`, `123abc`, `abc123`, `123abc456`)

* Start matching
  * `/^abc/` should only match strings starting with `abc` (e.g., `abc`, `abc123`)

* End matching
  * `/abc$/` should only match strings ending with `abc` (e.g., `abc`, `123abc`)

* Exact matching
  * `/^abc$/` should only match the string `abc`

* Case-insensitive matching
  * `/^abc$/i` should only match with `abc`, `Abc`, `aBc`, `abC`, `ABc`, `aBC`, `AbC`, `ABC`

* Optional matching
  * `/^ab?c$/` should only match with `abc`, `ac`

* Zero or more matching
  * `/^ab*c$/` should only match with `ac`, `abc`, `abbc`, `abbbc` and so on

* One or more matching
  * `/^ab+c$/` should only match with `abc`, `abbc`, `abbbc` and so on

* Numbered matching
  * `/^ab{2}c$/` should only match with `abbc`
  * `/^ab{3,}c$/` should only match with `abbbc`, `abbbbc`, `abbbbbc` and so on
  * `/^ab{2,4}c$/` should only match with `abbc`, `abbbc`, `abbbbc`

* Alternative matching
  * `/^ab|cd$/` should only match with `ab` and `cd`

* Any character matching
  * `/^.$/` should only match with `a`, `b`, `A`, `B`, `?` and so on

* Character range matching
  * `/^[abc]$/` should only match with `a`, `b` and `c`
  * `/^[a-d]$/` should only match with `a`, `b`, `c` and `d`

* Character range not matching
  * `/^[^abc]$/` should only **not** match with `a`, `b` and `c`
  * `/^[^a-d]$/` should only **not** match with `a`, `b`, `c` and `d`

* Escaping special characters
  * `/^\.$/` should only match with `.`
  * `/^\*$/` should only match with `*`
  * Same for all special characters used above (e.g., `[`, `]`, `$` and so on)

* And any combination of the features above!

Your implementation should comply with the following:
* You are only expected to write boolean `match` logic, no need to determine match location or matched substring
* You can use external dependencies
* You should detect non ASCII bytes and raise/return appropriate error

Your PR should comply with the following:
* For Concrete Numpy:
  * Create the script `examples/regex-engine.py`
  * Create the tutorial `docs/tutorial/regex-engine.{md,ipynb}`
* For TFHE-rs:
  * Create the script `tfhe/examples/regex-engine.rs`
  * Create the tutorial `tfhe/docs/tutorial/regex-engine.md`

## Library targeted
* [Concrete-Numpy](https://github.com/zama-ai/concrete-numpy)
* [TFHE-rs](https://github.com/zama-ai/tfhe-rs)

## Bounty type
[Expert bounty](https://github.com/zama-ai/bounty-program#expert-bounties)

## Reward
Up to €10,000 depending on performance

## Related links and references
* [Concrete-Numpy documentation](https://docs.zama.ai/concrete-numpy)
* [Concrete-Numpy contribution guide](https://docs.zama.ai/concrete-numpy/developer/contributing)
* [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs)
* [TFHE-rs contribution guide](https://docs.zama.ai/tfhe-rs/developers/contributing)

## Submission
Apply directly to this bounty by sending an application [here](https://zama.ai/bounty-program-application).

## Questions?
Do you have a specific question about this bounty? Join the live conversation on the FHE.org discord server [here](https://discord.fhe.org). You can also send us an email at: bounty@zama.ai
