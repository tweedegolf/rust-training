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

A language empowering everyone
to build reliable and efficient software.

## Python to Rust Interop Intro

<img src="https://raw.githubusercontent.com/tweedegolf/slidev-theme-tweedegolf/1bc81d09e326fcecb531108a5a3bcd9e1856dd84/images/shield-large.png" class=bg-image>

---
layout: trainer-intro
---

# Who are we?

## Tamme Dittrich

<br/>

- Embedded software engineer & Rust-lang trainer
- Worked on ntpd-rs, statime, and [drive-rs](https://github.com/tweedegolf/drive-rs)
- Worked in applied research in machine vision systems
- Taught the Embedded Rust Advanced class at EW24

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/tamme400x400.jpg" alt="Photo Tamme"/>
</center>
---
layout: trainer-intro
---

# Who are we?

## Folkert de Vries

<br/>

- Software engineer & Rust-lang trainer
- Worked on NTP, PTP, P2000, zlib-rs
- Contributor Roc programming language
- Talk to me about compilers, linkers, functional & systems programming

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/folkert.jpg" alt="Photo Folkert"/>
</center>

---
layout: with-footer
---

# A quick note about Tweede golf

- **Leading Rust experts** Running Rust in production since 2017
- **Engineers** Web, systems, embedded software development
- **Open source** Impactful implementations of Network Time Protocol, sudo
- **Adoption** Through both teach-rs and in-company training
- **Community** Organizing RustNL and active in Rust meetups

<p></p>

<center>
  <img class="logo"       src="https://tweedegolf.nl/images/client-bluebird-v2.png" alt="Bluebird">
  <img class="logo-smaller" src="https://tweedegolf.nl/images/kelvin-logo.png" alt="Kelvin">
  <img class="logo-small" src="https://tweedegolf.nl/images/client-dutch-iot.jpg" alt="DIS">
  <img class="logo-small" src="https://tweedegolf.nl/images/client-technolution.png" alt="Technolution">
  <img class="logo-smaller" style="width: 75px" src="https://tweedegolf.nl/images/gama-logo2.png" alt="GAMA">
  <br>
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-ferrous-BiR_kvPj.png" alt="Ferrous">
  <img class="logo-small" src="https://tweedegolf.nl/images/client-isrg.jpg" alt="ISRG">
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-nlnetlabs-CCcsDspS.png" alt="NLNet">
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-rustfoundation-Chh3nH12.png" alt="Rust Foundation">
</center>

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

# Training format

```rust
fn training() {
    let t = thread::spawn(|| loop {
        let question = Question::read_from_crowd();
        question.answer();
    });


    for part in training.parts() {
        part.slides();
        part.exercises();
    }

    t.join();
}
```

<br/>

### Ask questions anytime!

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
layout: cover
---

# Task 1: `count_kmers`

---
layout: cover
---

# Task 2: Throw an exception

---
layout: cover
---

# Task 3: `class` to `struct`

---
layout: cover
---

# Task 4: `dataclass` to `struct`

---
layout: cover
---

# (Stretch) Task 5: Iterators

---
layout: with-footer
---

# Evaluation

A 5 minute survey

- Share your thoughts - help us improve!
- Anonymously if desired

<https://training.tweede.golf/wrap-up.html>

---
layout: with-footer
---

# Reminder

Installation instructions: <https://training.tweede.golf>
