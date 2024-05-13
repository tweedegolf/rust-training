---
theme: tweedegolf
lineNumbers: true
fonts:
  mono: 'IBM Plex Mono'
  sans: 'IBM Plex Sans'
drawings:
  persist: false
layout: cover
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
 
## Henk Oordt
<br/>

- Embedded software engineer & Rust trainer
- 5 year Rust experience
- Work on IoT devices using Rust
- Maintainer of teach-rs

::right::
<center>
  <img class="face" src="https://tweedegolf.nl/images/screenshot-from-2024-03-01-15-47-57.png" alt="Photo Henk"/>
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
layout: with-footer
---

# A quick note about Tweede golf

- **Leading Rust agency in NL** Running Rust in production since 2017
- **Working on Rust adoption** Through both teach-rs and in-company trainings
- **Working on Rust projects** Like Rust implementations of PTP, NTP, sudo and su
- **Active community member** Organizing RustNL 2023 and active in various Rust meetups

<br/><br/>

<center>
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-ferrous-Ykf5Lz41.png" alt="Ferrous">
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-dutch-iot-0SWpC6-d.jpg" alt="DIS">
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-nlnetlabs-gnLA7KUl.png" alt="NLNet">
  <img class="logo-small" src="https://tweedegolf.nl/assets/client-technolution-s3BzJUDL.png" alt="Technolution">
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
let t = thread::spawn(|| loop {
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


