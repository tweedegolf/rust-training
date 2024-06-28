---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 8.3: Async on Embedded"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: "Rust - 8.3: Async on Embedded"
---

<style type="text/css" rel="stylesheet">
table.emb-overview {
    table-layout: fixed;

    td {
        border: 1px solid rgb(156 163 175 / 0.2);
    }
    tr.hide {
        visibility: hidden;
        border: 0;

        * {
            visibility: hidden;
            border: 0;
        }
    }
}

table.horizontal {
    table-layout: fixed;
    tr {
        border: 0;
    }
}
</style>

# Rust programming

Module 8: Rust for embedded

## Unit 3

Async on Embedded

---
layout: with-footer
---

# So far

- `embedded-hal`
- HALs, PACs
- Drivers
- Runtimes

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr>
    <td colspan="6"><center>Driver<br/>⬇️</center></td>
</tr>
<tr>
    <td colspan="6"><center><pre>embedded-hal</pre></center></td>
</tr>

<tr>
    <td colspan="1"><center>⬆️<br/>HAL<br/><pre>atsamd-hal</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>embassy-nrf</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>embassy-stm32</pre></center></td>
    <td colspan="1"><center>⬆️<br/>HAL<br/><pre>rp-hal</pre></center></td>
</tr>

<tr>
    <td><center>⬇️<br/>PAC<br/><pre>SAMD21E</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>nRF52833</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>nRF9160</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32H743</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32L476</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>RP2040</pre></center></td>
</tr>

</table>

---
layout: with-footer
---

# So far

- `embedded-hal`
- HALs, PACs
- Drivers
- Runtimes

**All sync!**

---
layout: with-footer
---

# What if you wanna multitask?

- Networking
- Saving power
- Do things in real time

<v-click><br/>

<center>

## Suggestions?

</center>

</v-click>

---
layout: with-footer
---

# Some ideas

- Roll your own interrupt-based state machine
- Use an RTOS - the C way
- Event loop
- [RTIC](https://rtic.rs)

<v-click>

- 🥁
</v-click>


<v-click><center>

## 🥁🥁🥁🥁🥁

</center></v-click>


---
layout: cover
---
<center>

## enter

# `async`/`await`

Let rustc do the job!

</center>

---
layout: with-footer
---

# Async vs RTOS

##### Or: threads vs futures

| Thread           | `Future`                 |
| ---------------- | ------------------------ |
| pre-emptive      | co-operative             |
| separate stack   | shared stack             |
| OS concept       | language/library concept |
| high OS coupling | bring your own executor  |

---
layout: with-footer
---

# `core::future::Future`

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

<v-click><center><br/><br/>

## Huh? `poll`? On embedded?

</center></v-click>

---
layout: with-footer
---

# `impl Context`

```rust
impl<'a> Context<'a> {
    /* ✂️✂️ */
    pub const fn waker(&self) -> &'a Waker {
        &self.waker
    }
    /* ✂️✂️ */
}
```

<v-click>

[Docs.rs on `Waker`](https://doc.rust-lang.org/stable/std/task/struct.Waker.html):
> A Waker is a handle for waking up a task by notifying its executor that it is ready to be run.

</v-click>

---
layout: cover
---

# `Future`s
You only `poll` 'em if you have to

---
layout: with-footer
---

# BYOE (Bring your own Executor)

- `tokio`
- `smol`
- `rtic`
- `embassy-executor`

---
layout: with-footer
---

# `embassy-executor`

Highlights:
- No `alloc`
- Integrated timer queue
- No busy polling
- Fair

---
layout: full
---

<figure style="height: 100%; margin: 0 auto;">
<img src="https://embassy.dev/book/images/embassy_executor.png" style="height: 95%; margin: 0 auto;"/>
<figcaption><center>

*Embassy's main function (https://embassy.dev/book/#_executor_2)*

</center></figcaption>
</figure>

---
layout: full
---

<figure style="height: 100%; margin: 0 auto;">
<img src="https://embassy.dev/book/images/embassy_irq.png" style="height: 95%; margin: 0 auto;"/>
<figcaption><center>

*Embassy's interrupt handlers (https://embassy.dev/book/#_interrupts)*

</center></figcaption>
</figure>


---
layout: cover
---

# Using `embassy-executor`


---
layout: with-footer
---

# Blinky 💡

<a href="https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/blinky.rs" target="_blank">

```rust{all|9-10|16,18|all}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
```

</a>

---
layout: with-footer
---

# `#[embassy_executor::task]` & [`Channel`](https://docs.rs/embassy-sync/latest/embassy_sync/channel/struct.Channel.html)

<a href="https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/channel.rs" target="_blank">

```rust{all|1,2|11,14|4-7,15-20|all}
#[embassy_executor::task]
async fn my_task() {
    loop {
        CHANNEL.send(LedState::On).await;
        Timer::after_secs(1).await;
        CHANNEL.send(LedState::Off).await;
        Timer::after_secs(1).await;
    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(my_task()));
    loop {
        match CHANNEL.receive().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
```

</a>

---
layout: with-footer
---

# Practice time!

&nbsp;

Unit 8.3 exercise description: [training.tweede.golf](https://training.tweede.golf/async-on-embedded.html)

*Don't forget to* `git pull`!
