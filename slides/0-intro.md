---
theme: tweedegolf
lineNumbers: true
info: "Rust 101: Course intro"
fonts:
  mono: 'IBM Plex Mono'
  sans: 'IBM Plex Sans'
drawings:
  persist: false
layout: cover
title: 'Rust - 0: Introduction'
---


# Rust programming

A language empowering everyone
to build reliable and efficient software.

## Workshop intro
<img src="https://raw.githubusercontent.com/tweedegolf/slidev-theme-tweedegolf/1bc81d09e326fcecb531108a5a3bcd9e1856dd84/images/shield-large.png" class=bg-image>

---
layout: with-footer
---

# A quick note about Tweede golf

- **Leading Rust agency in NL** <span class="light">running Rust in production since 2017</span>
- **Working on Rust adoption** <span class="light">through both Rust 101 and in-company workshops</span>
- **Working on Rust projects** <span class="light">like Rust implementations of PTP, NTP, sudo and su</span>
- **Active community member** <span class="light">organizing RustNL 2023 and active in various Rust meetups</span>

<br/><br/>

<center>
  <img class="logo-small" src="https://tweedegolf.nl/06f290cc0f37.png" alt="Ferrous">
  <img class="logo-small" src="https://tweedegolf.nl/21df02fc203f.png" alt="GAMA">
  <img class="logo-small" src="https://tweedegolf.nl/0e1499aea7d7.jpg" alt="DIS">
  <img class="logo-small" src="https://tweedegolf.nl/ca5589b58a30.png" alt="NLNet">
  <img class="logo-small" src="https://tweedegolf.nl/1495c30ec80b.png" alt="Technolution">
</center>

---
layout: two-cols
---

# Who are we?

## Henk Oordt
<br/>

- Embedded software engineer & Rust-lang trainer
- 4 year Rust experience
- Work on IoT devices using Rust
- Maintainer of Rust 101

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/henk.jpg" alt="Photo Henk"/>
</center>
---
layout: two-cols
---

# Who are we?

##  Dion Dokter
<br/>

- Embedded tech lead & Rust-lang trainer
- Joined Tweede golf in 2021
- Full-time Rust since 2019
- Worked with LoRaWAN, UWB, LTE
- Created stackdump, nrf-modem & device-driver crates

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/dion.jpg" alt="Photo Dion"/>
</center>

---
layout: two-cols
---

# Who are we?

##  Folkert de Vries
<br/>

- Software engineer & Rust-lang trainer
- Joined Tweede golf in 2021
- Worked on NTP, PTP, P2000
- Contributor Roc programming language
- Created stackdump, nrf-modem & device-driver crates
- Talk to me about compilers, linkers, functional & systems programming

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/folkert.jpg" alt="Photo Folkert"/>
</center>

---
layout: two-cols
---

# Who are we?

##  Marc Schoolderman
<br/>

- Software engineer & former CS teacher
- Joined Tweede golf in 2022
- Worked on Sudo-rs
- Taught and did CS research at Radboud University’s Security Group

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/marcvoortgwebsite.jpg" alt="Photo Marc"/>
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


- **Multi-purpose** <span class="light">Build high-quality software, from cloud embedded and systems</span>
- **Core strenghs** <span class="light">"C-like" performance, with memory safety and modern ergonomics</span>
- **Confident developers** <span class="light">SO's most admired (loved) language for 8 years in a row</span>
- **Momentum** <span class="light">Adopted by big tech: Chromium, Linux & Windows kernels</span>

---
layout: with-footer
---

# Are there downsides too?

Sure there are. The things mentioned most often:

- **Steep learning curve** <span class="light">Getting *productive* with Rust takes relatively little time, though</span>
- **Relatively new ecosystem** <span class="light">Do your research before getting started</span>
- **Compiling large projects can be slow** <span class="light">Similar to template-heavy C++</span>

---
layout: with-footer
---
# Workshop format

```rust
let t = thread::spawn(|| loop {
    let question = Question::read_from_crowd();
    question.answer();
})


for part in workshop.parts() {
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

# Reminder

*Installation instructions: <https://workshop.tweede.golf>*

Follow them now if you haven't yet!

