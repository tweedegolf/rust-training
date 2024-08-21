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

## Dion Dokter

<br/>

- Embedded tech lead & Rust-lang trainer
- Full-time Rust since 2019
- Worked with LoRaWAN, UWB, LTE, PTP
- Created sequential-storage, device-driver & nrf-modem crates
- Contributor to the Rust compiler and Embassy

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/dion.jpg" alt="Photo Dion"/>
</center>

---
layout: trainer-intro
---

# Who are we?

## Folkert de Vries

<br/>

- Software engineer & Rust-lang trainer
- Worked on NTP, PTP, P2000
- Contributor Roc programming language
- Talk to me about compilers, linkers, functional & systems programming

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/folkert.jpg" alt="Photo Folkert"/>
</center>

---
layout: trainer-intro
---

# Who are we?

## Marc Schoolderman

<br/>

- Software engineer & Rust-lang trainer
- Worked on Sudo-rs, and ntpd-rs
- Formerly part of Radboud University’s Security Group
- Security officer worried about supply chain risks
- Talk to me about programming languages and formal verification

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/marcvoortgwebsite.jpg" alt="Photo Marc"/>
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


