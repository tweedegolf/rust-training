# Unit 7.1 - Rust from Python

<a href="/slides/7_1-rust-from-python/" target="_blank">Slides</a>

## Exercise 7.1.1: Test your environment

Install uv (https://docs.astral.sh/uv/#installation) and Rust (https://www.rust-lang.org/tools/install)

Check it is working:
```bash
cd rust-dna
uv run maturin develop --uv
cd ..
uv run pytest
```

The output should look something like this:
```
❯ uv run pytest
Installed 5 packages in 9ms
============================= test session starts ==============================
platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.5.0
rootdir: /home/tamme/dev/rust-training/exercises/7-rust-for-data-science/1-rust-from-python
configfile: pyproject.toml
collected 10 items

test_decoding.py ....                                                    [ 40%]
test_kmers.py ....                                                       [ 80%]
test_setup.py .                                                          [ 90%]
test_validation.py .                                                     [100%]

============================== 10 passed in 0.06s ==============================
```

## Exercise 7.1.2: Translating Python to rust

Follow the steps in `exercises/7-rust-for-data-science/1-rust-from-python/main.py` to translate the logic to Rust in `exercises/7-rust-for-data-science/1-rust-from-python/rust-dna/lib.rs`.
