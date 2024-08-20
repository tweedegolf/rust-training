---
theme: tweedegolf
lineNumbers: true
fonts:
  mono: 'IBM Plex Mono'
  sans: 'IBM Plex Sans'
drawings:
  persist: false
layout: cover
routerMode: hash
---

# Rust programming

A language empowering everyone
to build reliable and efficient software.

## Training intro

<img src="https://raw.githubusercontent.com/tweedegolf/slidev-theme-tweedegolf/1bc81d09e326fcecb531108a5a3bcd9e1856dd84/images/shield-large.png" class=bg-image>

---
layout: two-cols
---

# Who are we?
 
## Tamme Dittrich
<br/>

- Embedded software engineer & Rust-lang trainer
- Joined Tweede golf in 2023
- Worked on ntpd-rs, statime, and [drive-rs](https://github.com/tweedegolf/drive-rs)
- Contributor to the Rust compiler
- Worked in applied research in machine vision systems

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/tamme400x400.jpg" alt="Photo Tamme"/>
</center>
---
layout: two-cols
---

# Who are we?

## Dion Dokter

<br/>

- Embedded tech lead & Rust-lang trainer
- Joined Tweede golf in 2021
- Full-time Rust since 2019
- Worked with LoRaWAN, UWB, LTE, PTP
- Created sequential-storage, device-driver & nrf-modem crates
- Contributor to the Rust compiler and Embassy

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/dion.jpg" alt="Photo Dion"/>
</center>

---
layout: two-cols
---

# Who are we?

## Folkert de Vries

<br/>

- Software engineer & Rust-lang trainer
- Joined Tweede golf in 2021
- Worked on NTP, PTP, P2000
- Contributor Roc programming language
- Talk to me about compilers, linkers, functional & systems programming

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/folkert.jpg" alt="Photo Folkert"/>
</center>

---
layout: two-cols
---

# Who are we?

## Marc Schoolderman

<br/>

- Software engineer & Rust-lang trainer
- Joined Tweede golf in 2022
- Worked on Sudo-rs, and ntpd-rs
- Taught and did CS research at Radboud University’s Security Group

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/marcvoortgwebsite.jpg" alt="Photo Marc"/>
</center>

---
layout: with-footer
---

# A quick note about Tweede golf

- **Leading Rust agency in NL** Running Rust in production since 2017
- **Working on Rust adoption** Through both teach-rs and in-company trainings
- **Working on Rust projects** Like Rust implementations of PTP, NTP, sudo and su
- **Active community member** Organizing RustNL 2023 and active in various Rust meetups

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

# Why learn Rust?

Q: What do you know about Rust?

---
layout: with-footer
---

# What Rust brings

- **Multi-purpose** Build high-quality software, from cloud embedded and systems
- **Core strenghs** "C-like" performance, with memory safety and modern ergonomics
- **Confident developers** SO's most admired (loved) language for 8 years in a row
- **Momentum** Adopted by big tech: Chromium, Linux & Windows kernels

---
layout: with-footer
---

# Are there downsides too?

Sure there are. The things mentioned most often:

- **Steep learning curve** Getting *productive* with Rust takes relatively little time, though
- **Relatively new ecosystem** Do your research before getting started
- **Compiling large projects can be slow** Similar to template-heavy C++

---
layout: with-footer
---

# Training format

```rust
let t = thread::spawn(| | loop {
    let question = Question::read_from_crowd();
    question.answer();
})


for part in training.parts() {
    part.slides();
    part.exercises();
}

t.join();

```

<br/>

### Ask questions anytime!

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


