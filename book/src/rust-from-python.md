# Unit 7.1 - Rust from Python

<a href="/slides/7_1/" target="_blank">Slides</a>

## Exercise 7.1.1: Test your environment

### Exercise 7.1.1.A: Set up the tools

Follow the instructions to instal the Maturin build tool: <https://pyo3.rs/v0.21.2/getting-started>. We recommend using Pyenv:
<https://github.com/pyenv/pyenv>, but Pyenv only supports UNIX and WSL. Of course, you can use your favourite venv manager, too.

Navigate to the `exercises/7-rust-for-data-science/1-rust-from-python/1-hello-world/` folder in your terminal. Setup and activate your virtual environment, and install the things we need. If you're using pyenv, that means running the following commands:

```shell
$ pyenv activate pyo3
$ pip install maturin
$ pip install asyncio
$ pip install aiofiles
```

### Exercise 7.1.1.B: Trying it out

If everything went well, you should now be able to build this project into a Python extension:

```shell
$ maturin develop
$ python test.py
```

**The `test.py` script ends in a call to a function that unconditionally raises, so your script is expected to result in an uncaught exception.**

Now, try running `test_async.py` as well and observe its behavior.

### Exercise 7.1.1.C: Running futures concurrently
You can use [`asyncio.gather`](https://docs.python.org/3/library/asyncio-task.html#asyncio.gather) in your python script to execute multiple awaitables. Spawn a number of `print_sleep` futures, each with different sleep durations using `asyncio.gather` and observe the behavior.
