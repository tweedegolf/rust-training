---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 7.1: Calling Rust from Python"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 7.1: Calling Rust from Python"
routerMode: hash
---

# Rust programming

Module 7: Rust for data science

## Unit 1

Calling Rust from Python

---
layout: cover
---

## Q: Why call Rust libraries from Python applications?

---
layout: default
---
# Some reasons to call Rust

- Get a free ecosystem, but keep access to familiar tools
- Get performance, without the footguns from C or C++
- Multithreading
- Cause it's fun!

---
layout: cover
---

# So how do you do it?
Q: What would you need to have python call Rust? 

---
layout: default
---

# Enter PyO3
<br/>

> Rust bindings to the Python interpreter.

<br/><br/>
<v-click>
Use it to

- Write native Python modules in Rust
- ~~Run python code and modules from Rust~~

</v-click>
<br/><br/>
<v-click>
<center>

### User guide: [https://pyo3.rs](https://pyo3.rs)
</center>
</v-click>

---
layout: three-slots
---

# What do you mean, 'bindings'?
::left::
<v-click>

### Type definitions
- Numbers
- Lists
- Strings
- Dictionaries
- Exceptions
<br/><br/>
</v-click>

<v-click>

### Functions
- Interface
- Definition
</v-click>
::right::

<v-click>

### Module
- Definition
- Submodules
<br/><br/>
</v-click>

<v-click>

### Classes
- Magic functions
<br/><br/>
</v-click>

<v-click>

### Memory model
- Reference counting
- GIL vs lifetimes
</v-click>

<!-- 
Basically, PyO3 allows you to use Python concepts from Rust
-->

---
layout: default
---

# Creating a Python extension in 6 simple steps
<v-click>

1. **Build a Rust library**

*or pick an existing one*
</v-click><v-click>

2. **Write wrapper crate/module**

*expose an API for Python*
</v-click><v-click>

3. **Build it with `maturin develop`**

> Build and publish crates with pyo3, [...] bindings [...] as python packages with minimal configuration.

</v-click><v-click>

4. **Call it from python**
<br/><br/>
</v-click><v-click>

5. **Publish using maturin or setuptools-rust**
<br/><br/>
</v-click><v-click>

6. **💰 Profit!**
</v-click>

---
layout: cover
---

# Let's see some code!

---
layout: default
---

# Crate setup

```toml
[package]
name = "hello-py"
version = "0.1.0"
edition = "2021"

[lib]
name = "hello_py"               # Name of your module
crate-type = ["cdylib", "lib"]  # Usable from outside as well as from Rust

[dependencies.pyo3]
version = "0.21.0"
features = ["extension-module"]
```

---
layout: with-footer
---

# Hello, world!

```rust
use pyo3::prelude::*;

#[pyfunction]
pub fn say_hello() {
    println!("Hello world!");
}

#[pymodule]
fn hello_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
```

---
layout: with-footer
---

# Build it

```shell
$ pyenv virtualenv pyo3
$ pyenv activate pyo3
$ pip install maturin
$ maturin develop
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /home/hd/.pyenv/versions/pyo3/bin/python
   Compiling pyo3-build-config v0.21.1
   Compiling pyo3-macros-backend v0.21.1
   Compiling pyo3-ffi v0.21.1
   Compiling pyo3 v0.21.1
   Compiling pyo3-macros v0.21.1
   Compiling hello-py v0.1.0 (/home/hd/dev/tg/edu/rust-training/exercises/7-rust-for-data-science/1-rust-from-python/1-hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 6.84s
📦 Built wheel for CPython 3.12 to /tmp/.tmpWV4WTB/hello_py-0.1.0-cp312-cp312-linux_x86_64.whl
✏️  Setting installed package as editable
🛠 Installed hello-py-0.1.0
```

---
layout: with-footer
---

# Run it

```python{all|3|4|5}
Python 3.12.2 (main, Feb 21 2024, 00:00:00) [GCC 13.2.1 20231205 (Red Hat 13.2.1-6)] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> from hello_py import say_hello
>>> say_hello()
Hello world!
```

<v-click><center>
<br/><br/>

## **💰 Profit!**

</center></v-click>
---
layout: cover
---

# Memory management

---
layout: with-footer
---

# Hello again!

```rust{all|9}
use pyo3::prelude::*;

#[pyfunction]
pub fn say_hello() {
    println!("Hello world!");
}

#[pymodule]
fn hello_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
```

---
layout: three-slots
---

# Python vs Rust
[source](https://www.teach.cs.toronto.edu/~csc110y/fall/notes/06-memory-model/04-python-memory-model-1.html)

<br/><br/>

::left::
## Python

- Variable = reference to heap value
- Garbage collected (multi threaded GC)
- All values are reference counted
- GIL protects against data races

::right::

## Rust

- Variable = data or reference
- Drop on out of scope
- Ref count is opt-in
- Borrow checker protects against data races

---
layout: with-footer
---

# Smart pointers to the rescue!

```rust{2}
#[pymodule]
fn hello_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
```

- `Python`: GIL token: exclusive access to Python heap
- `&Bound<'_, T>`: Smart reference counted pointer to `T` on Python heap. Wraps a `Python`

<v-click>

- `Py<T>`: Smart reference counted pointer to `T` on Python heap. Access requires `Python`
</v-click>

---
layout: cover
---

# More features
Making life easier

---
layout: with-footer
---

# Classes

Define the data, getters, and setters

```rust{all|3,4|3-9}
use pyo3::prelude::*;

#[pyclass]                                                      // Expose as Python class
pub struct Point {
    #[pyo3(get, set)]                                           // Generate getters and setters
    x: u32,
    #[pyo3(get, set)]
    y: u32,
}

```

---
layout: with-footer
---

# Classes (2)

Add the methods

```rust{all|1,2|3-6|8-11|13-15}
#[pymethods]
impl Point {
    #[new]                                                     // Mark as constructor
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn dist(&self, rhs: &Self) -> f32 {                         // 'normal' method
        ((self.x as f32 - rhs.x as f32).powi(2) +
            (self.y as f32 - rhs.y as f32).powi(2)).sqrt()
    }

    fn __str__(slf: PyRef<'_, Self>) -> String {                // Magic method
        format!("Point {{x = {}, y = {}}}", slf.x, slf.y)
    }
}
```

---
layout: with-footer
---

# Classes (3)

Add it to your module

```rust{all|5}
#[pymodule]
fn hello_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(fact, m)?)?;
    m.add_class::<point::Point>()?;
    Ok(())
}
```

- [`::<>`: Turbofish syntax](https://turbo.fish)

---
layout: with-footer
---

# Classes (4)

Try it out!

```python{all|5,6|7,8|9,10|11-13}
$ python
Python 3.12.2 (main, Feb 21 2024, 00:00:00) [GCC 13.2.1 20231205 (Red Hat 13.2.1-6)] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import hello_py
>>> point1 = hello_py.Point(1,1)
>>> point2 = hello_py.Point(0,2)
>>> print(point1)
Point {x = 1, y = 1}
>>> print(point1.dist(point2))
1.4142135381698608
>>> point2.x = 3
>>> print(point2.x)
3
```
---
layout: with-footer
---

# Things `#[pyclass]` can do

- Inheritance
- Getters/setters with side effects
- Static methods
- Class methods/attributes
- Enums
- Many more magic methods

[Have a look at the docs!](https://pyo3.rs/v0.21.1/class)

---
layout: with-footer
---

# Limitations of `#[pyclass]`

- No generics
- No lifetime parameters
- Must be `Send` by default

---
layout: cover
---

# Exceptions

---
layout: with-footer
---

# Exceptions

```rust{all|9|9-16}
use pyo3::prelude::*;

#[derive(Debug)]
pub enum HelloError {
    Io(std::io::Error),
    AppleStuckInThroat,
}

impl std::fmt::Display for HelloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelloError::Io(e) => write!(f, "Hello Error: {e}"),
            HelloError::AppleStuckInThroat => write!(f, "Hello Error: 🍏🍏"),
        }
    }
}
```

---
layout: with-footer
---

# Exceptions (2)

```rust{all|1|3|5-8|12-16}
impl From<HelloError> for PyErr {
    fn from(e: HelloError) -> Self {
        use pyo3::exceptions::*;

        match e {
            HelloError::Io(_) => PyIOError::new_err(e.to_string()),
            HelloError::AppleStuckInThroat => PyBaseException::new_err(e.to_string()),
        }
    }
}

#[pyfunction]
pub fn throws_error() -> PyResult<()> {
    Err(HelloError::AppleStuckInThroat)?;
    unreachable!()
}
```

---
layout: with-footer
---

# Exceptions (3)

```python
Python 3.12.2 (main, Feb 21 2024, 00:00:00) [GCC 13.2.1 20231205 (Red Hat 13.2.1-6)] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import hello_py
>>> hello_py.throws_error()
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
BaseException: Hello Error: 🍏🍏
```

---
layout: cover
---

# `async`/`await`


---
layout: with-footer
---

# `async`/`await`

Currently in active development

```toml{all|3}
[dependencies.pyo3]
version = "0.21.0"
features = ["extension-module", "experimental-async"]
```

---
layout: with-footer
---

# `async`/`await` (2)

```rust
#[pyfunction]
pub async fn print_sleep(duration: Duration) {
    let start = SystemTime::now();
    println!("🌙 Night night! Sleeping for {:?}", duration);
    sleep(duration).await;
    println!(
        "🌞 I'm awake after {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );
}
```

---
layout: with-footer
---

# `async`/`await` (2)

```python
python -m asyncio
asyncio REPL 3.12.2 (main, Feb 21 2024, 00:00:00) [GCC 13.2.1 20231205 (Red Hat 13.2.1-6)] on linux
Use "await" directly instead of "asyncio.run()".
Type "help", "copyright", "credits" or "license" for more information.
>>> import asyncio
>>> import hello_py
>>> import datetime
>>> duration = datetime.timedelta(seconds=3)
>>> await hello_py.print_sleep(duration)
🌙 Night night! Sleeping for 3s
🌞 I'm awake after 3.000735818s
```

---
layout: with-footer
---

# `Limitations of async`/`await`

- GIL held during execution of `Future`
- `Send` + `'static`: no references in signature

```rust{all|7-9}
#[pyfunction]
async fn does_not_compile<'py>(arg: Bound<'py, PyAny>) -> Bound<'py, PyAny> {
    todo!()
}

#[pyfunction]
async fn does_compile(arg: Py<PyAny>) -> Py<PyAny> {
    Python::with_gil(|py| { // (almost) no-op
        let r: &Bound<'_, PyAny> = arg.bind(py);

        todo!()
    })
}
```

---
layout: default
---

# Practice time!

&nbsp;

Unit 7.1 exercise description: [training.tweede.golf](https://training.tweede.golf/rust-from-python.html)

*Don't forget to* `git pull`!
