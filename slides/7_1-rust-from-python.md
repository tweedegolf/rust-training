---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 7.1: Rust from Python"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 7.1: Rust from Python"
routerMode: hash
---

# Rust programming

Module 7: Rust for Data Science

## Unit 1

Rust from Python

---
layout: cover
---

# Why use Rust in Python?

Q: Why do you use Rust and/or Python?

---
layout: with-footer
---

# Why use Rust in Python?

- **Speed**
- **Crate ecosystem**
- **Robustness**
- **Incremental Migration**

---
layout: with-footer
---

# Famous Examples

- **polars**
- **pydantic-core**
- **cryptography**
- **tantivy-py**

---
layout: with-footer
---

# Teaching goals

* Call Rust from Python
* Expose a Rust `struct` to Python
* Rust Iterators
* Some Bioinformatics

---
layout: cover
---

## Experience with Rust/Python/PyO3?

---
layout: with-footer
---

# What is PyO3?

```rust
#[pyfunction]
fn count_chars(s: &str) -> PyResult<usize> {
    Ok(s.chars().count())
}

#[pymodule]
fn rust_dna(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_chars, m)?)?;
    Ok(())
}
```

[pyo3.rs](https://pyo3.rs/)

---
layout: with-footer
---

# What is Maturin

```bash
# Install to local venv
maturin develop

# Install to local uv env
maturin develop --uv

# Build a Python wheel in release mode
maturin build --release
```

*Note:* Also check-out `new` and `generate-ci`

---
layout: with-footer
---

# Quick Intro: k-mers (n-grams)

<center>
<img src="https://upload.wikimedia.org/wikipedia/commons/8/8a/K-mer_diagram.svg" class="logo" />
</center>

Attribution: ["Group 3"](https://commons.wikimedia.org/wiki/File:K-mer_diagram.svg) by Ytngargar is licensed
under [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.en)

---
layout: default
---

# Practice time!

&nbsp;

Unit 7.1.2 exercise description: [training.tweede.golf](https://training.tweede.golf/foreign-function-interface.html)

*Don't forget to* `git pull`!

<!-- For further steps use the PyO3 docs live -->
