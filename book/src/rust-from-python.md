# Unit 7.1 - Rust from Python

<a href="/slides/7_1-rust-from-python/" target="_blank">Slides</a>

## Exercise 7.1.1: Test your environment

### 7.1.1.A: Set up the tools ⭐

Follow the instructions to instal the Maturin build tool: <https://pyo3.rs/v0.21.2/getting-started>. We recommend using pyenv:
<https://github.com/pyenv/pyenv>, but pyenv only supports UNIX and WSL. Of course, you can use your favourite venv manager, too.

Navigate to the `exercises/7-rust-for-data-science/1-rust-from-python/1-hello-world/` folder in your terminal. Setup and activate your virtual environment, and install the things we need. If you're using pyenv, that means running the following commands:

```shell
$ pyenv activate pyo3
$ pip install maturin
$ pip install asyncio
$ pip install aiofiles
```

### 7.1.1.B: Trying it out ⭐

If everything went well, you should now be able to build this project into a Python extension:

```shell
$ maturin develop
$ python test.py
```

**The `test.py` script ends in a call to a function that unconditionally raises, so your script is expected to result in an uncaught exception.**

Now, try running `test_async.py` as well and observe its behavior.

### 7.1.1.C: Running futures concurrently ⭐⭐
You can use [`asyncio.gather`](https://docs.python.org/3/library/asyncio-task.html#asyncio.gather) in your python script to execute multiple awaitables. Spawn a number of `print_sleep` futures, each with different sleep durations using `asyncio.gather` and observe the behavior.

## Exercise 7.1.2: Streaming JSON
Here's a cool crate: [Struson](https://github.com/marcono1234/struson). It's a crate that allows for (de)serialization of JSON objects in a streaming fashion, meaning that you don't need to hold all of the data in memory. This contrasts a bit with the great library [serde_json](https://github.com/serde-rs/json), which is very, very ergonomic, but does not allow for easy streaming (de)serialization of data. We're going build something around Struson, that we can use from Python.

Now, let's say we have a stream of JSON. Could be from someone on the internet or some other process running on the machine. Or, you know, a file. The stream consists of data that looks like this:

```json=
[
    {
        "lhs": {
            "d":[
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12
            ],
            "n": 3
        },
        
        "op": [
            {
                "code": "dot",
                "rhs": {
                    "d": [
                        13, 14, 15, 16,
                        17, 18, 19, 20,
                        21, 22, 23, 24
                    ],
                    "n": 3
                }
            }
        ]
    }
]
```

This JSON is an array of objects that each represent the following:
- `lhs`: a matrix with `m` rows and `n` columns. `m` can be derived from `n` and the length of `d`. In this case `m = len / n = 12 / 3 = 4`.
- `op`: a sequence of operations that need to take place given `lhs` and, if the operation takes two operands, its `rhs` field. `x` corresponds to the operation that should be executed, and should more or less correspond to the methods provided on [nalgebra](https://nalgebra.org/)'s [Matrix](https://docs.rs/nalgebra/0.32.4/nalgebra/base/struct.Matrix.html) type. In this case, the [`Matrix::dot`](https://docs.rs/nalgebra/0.32.4/nalgebra/base/struct.Matrix.html#dotscalar-product) method should be run, which, for probably good reason, is describead as 'the dot product between two vectors or matrices (seen as vectors)'.

Our library should streamingly deserialize each incoming object from an [`asyncio`](https://docs.python.org/3/library/asyncio.html) stream of bytes, apply the given operation, and pass on the result.


### Exercise 7.1.2.A: Give it a go ⭐
The scaffolding code in `exercises/7-rust-for-data-science/1-rust-from-python/2-strompy/` implements what we want in Rust, and also provides a synchronous implementation. Have a look around in the code, and try to make sense of it. <https://docs.rs> is your friend if you want to know more about the dependencies that are being used. Note that for `struson`, we import a fork that supports deserializing asynchronously.

You can run `cargo test` to build and run the tests in `src/lib.rs`.

### Exercise 7.1.2.B: The sync way ⭐⭐
During the next exercises, it's good to keep the [PyO3 User Guide](https://pyo3.rs/v0.21.2/) and the [PyO3 API docs](https://docs.rs/pyo3/0.21.2/pyo3/index.html) at hand.

This exercise is about making `strompy_test.py` run. Therefore, fix the `todo!()` in the `exec` function in `src/lib.rs`. Don't forget to add the function to the exposed module!

If you've done it correctly, you should be able to run it with the following commands:

```shell
$ maturin develop
$ python strompy_test.py
```

### 7.1.2.B: The `async` way ⭐⭐⭐
This time, we'll make `strompy_test_async.py` work. Implement `feed_bytes` and add an async method to the `StrompyJsonReader` type that yields a `PyResult<Option<Vec<Vec<f64>>>>` and is exposed with the name '`next`'.

If you've done it correctly, you should be able to run it with the following commands:

```shell
$ maturin develop
$ python strompy_test_async.py
```

### 7.1.2.C: More features (Bonus) ⭐⭐⭐
Can you make `strompy` support [more operations from Nalgebra](https://docs.rs/nalgebra/0.32.4/nalgebra/base/struct.Matrix.html)?

### 7.1.2.D: Tidying up (Bonus) ⭐⭐⭐⭐
Representing the vectors as `Vec<Vec<f64>>` works, but it's not great. Make the `StrompyJsonReader` produce [`PyResult<Option<Py<PyList>>>`](https://docs.rs/pyo3/0.21.2/pyo3/types/struct.PyList.html)s that constists of `PyList`s instead. 


*This is quite a hard exercise, as it involves working with PyO3s smart pointers: [`Py<T>`](https://docs.rs/pyo3/0.21.2/pyo3/struct.Py.html), [`Bound<'py, T>`](https://docs.rs/pyo3/0.21.2/pyo3/struct.Bound.html), and GIL tokens: [`Python`](https://docs.rs/pyo3/0.21.2/pyo3/marker/struct.Python.html)*
