---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 8.1: The Rust Embedded Ecosystem"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 8.1: The Rust Embedded Ecosystem"
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

## Unit 1

The Rust Embedded Ecosystem

---
layout: with-footer
---

# Exploring the Ecosystem

<table class="horizontal">
<tr>
    <th><center>🔸<br/><strong>Rust, C++, C</strong></center></th>
    <th><center>🔸<br/><strong>core, alloc, std</strong></center></th>
    <th><center>🔸<br/><strong>cargo, crates, rustc</strong></center></th>
    <th><center>🔸<br/><strong>probe-rs</strong></center></th>
</tr>
<tr>
    <td>
        <center><img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" width="75" height="75"></center>
        <center>
            <img style="display:inline-block" src="https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg" width="75" height="75">
            <img style="display:inline-block" src="https://upload.wikimedia.org/wikipedia/commons/1/19/C_Logo.png" width="75" height="75">
        </center>
    </td>
    <td></td>
    <td><center><img src="https://foundation.rust-lang.org/img/cargo.png" width="75" height="75"></center></td>
    <td><center><img src="https://probe.rs/images/banner.svg" width="75" height="75"></center></td>
</tr>
</table>

---
layout: three-slots
---

# Cortex-m crates

<br/><br/><br/>

::left::
## `cortex-m`
<br/>

- Peripheral Access Crate (PAC)
- Similar to CMSIS register definitions

::right::

## `cortex-m-rt`
<br/>

- Startup Runtime
- Interrupt setup

---
layout: with-footer
---

# Device PACs

Every device has different peripherals. One PAC for every device.

- Generated from SVD
- Imported as dependency

---
layout: with-footer
---

# Device PACs

### C
```c
#include "samd21e17l.h"

// Raw
bool is_8_cycles = ((WDT->CONFIG.reg & WDT_CONFIG_PER_Msk) << WDT_CONFIG_PER_Pos) == WDT_CONFIG_PER_8_val;
WDT->CONFIG.reg = (WDT->CONFIG.reg & ~WDT_CONFIG_PER_MSK) | WDT_CONFIG_PER_16;

// Bitfield
bool is_8_cycles = WDT->CONFIG.bit.PER == WDT_CONFIG_PER_8_val;
WDT->CONFIG.bit.PER = WDT_CONFIG_PER_16;
```

### Rust
```rust
// Take ownership of the peripherals
let dp = atmsamd21e::Peripherals::take().unwrap();

let is_8_cycles = dp.WDT.CONFIG.read().per().is_8();
dp.WDT.CONFIG.modify(|_, w| w.per()._8());
```

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr class="hide">
    <td colspan="7"><center><br/>Driver</center></td>
</tr>
<tr class="hide">
    <td colspan="7"><center><br/><pre>embedded-hal</pre></center></td>
</tr>

<tr class="hide">
    <td colspan="1"><center><br/>HAL<br/><pre>atsamd</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>nrf-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>stm32h7xx-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>stm32l4xx-hal</pre></center></td>
</tr>

<tr>
    <td><center><br/>PAC<br/><pre>nRF52840</pre></center></td>
    <td><center><br/>PAC<br/><pre>nRF9160</pre></center></td>
    <td><center><br/>PAC<br/><pre>SAMD21E</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32H743</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32H753</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32L476</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32L496</pre></center></td>
</tr>

</table>

---
layout: with-footer
---

# Device HALs

- Many open source HALs
- Most basic operations supported
- Built on top of PACs

---
layout: full
---

```rust
#[entry]
fn main() -> ! {
    // Take the device's peripherals
    let dp = Peripherals::take().unwrap();

    // Create the timer and give it access to the peripheral
    let mut timer = Timer::periodic(db.TIMER0);
    timer.enable_interrupt();
    timer.start(1_000_000u32); // Timer runs at 1MHz, so it will interrupt every second
    drop(timer); // Explicitly drop the timer

    // Unmask the timer interrupt in the NVIC. To convince the compiler this is actually
    // safe to do in this case, we do this inside an `unsafe` block
    unsafe { NVIC::unmask(Interrupt::TIMER0); }

    loop {}
} 

#[interrupt]
fn TIMER0 {
    // Get a reference to the peripheral
    // This is unsafe because only one instance may exist at a time or we'll trigger UB.
    // This is OK here, as we explicitly dropped the tiemr in main.
    // However, there are better ways to share state between contexts
    let timer = unsafe { &*TIMER0::ptr() };
    timer.events_compare[0].write(|w| w); // Clear the interrupt flag
}
```

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr class="hide">
    <td colspan="7"><center><br/>Driver</center></td>
</tr>
<tr class="hide">
    <td colspan="7"><center><br/><pre>embedded-hal</pre></center></td>
</tr>

<tr>
    <td colspan="1"><center><br/>HAL<br/><pre>atsamd</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>nrf-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>stm32h7xx-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>stm32l4xx-hal</pre></center></td>
</tr>

<tr>
    <td><center>⬇️<br/>PAC<br/><pre>nRF52840</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>nRF9160</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>SAMD21E</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32H743</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32H753</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32L476</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32L496</pre></center></td>
</tr>

</table>

---
layout: with-footer
---
# `embedded-hal`

The glue of the entire ecosytem
- Contains abstractions for many common operations

<br/>

### SPI example trait:
```rust
pub trait Transfer<W> {
    type Error;

    fn transfer<'w>(
        &mut self,
        words: &'w mut [W]              // ◄─┐
    ) -> Result<&'w [W], Self::Error>;  // ◄─┴─ 'w: returned slice lives as long as `words`
}
```

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr class="hide">
    <td colspan="7"><center><br/>Driver</center></td>
</tr>
<tr>
    <td colspan="7"><center><br/><pre>embedded-hal</pre></center></td>
</tr>

<tr>
    <td colspan="1"><center>⬆️<br/>HAL<br/><pre>atsamd</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>nrf-hal</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>stm32h7xx-hal</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>stm32l4xx-hal</pre></center></td>
</tr>

<tr>
    <td><center>⬇️<br/>PAC<br/><pre>nRF52840</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>nRF9160</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>SAMD21E</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32H743</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32H753</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32L476</pre></center></td>
    <td><center>⬇️<br/>PAC<br/><pre>STM32L496</pre></center></td>
</tr>

</table>

---
layout: with-footer
---

# Typestate

State encoded in the *type* of the variable

```rust
use nrf52840_hal::gpio::{Pin, p0::P0_04, Input, PullDown, Output, PushPull};

/// Take an nRF pin.
/// It must be:
/// - Port 0 pin 4 (Compile time constant)
/// - Configured as input
/// - Pulldown enabled
fn do_something_1(pin: P0_04<Input<PullDown>>) {}

/// Take an nRF pin.
/// It must be:
/// - Any port and pin (Runtime variable)
/// - Configured as output
/// - Configured as push-pull
fn do_something_2(pin: Pin<Output<PushPull>>) {}
```

---
layout: with-footer
---

# Runtimes

<table class="horizontal">
<tr>
    <th><center>🔸<br/><strong>Bare metal<br/>+ interrupts</strong></center></th>
    <th><center>🔸<br/><strong>RTIC<br/>(Real-Time Interrupt-<br/>driven Concurrency)</strong></center></th>
    <th><center>🔸<br/><strong>RTOS</strong></center></th>
    <th><center>🔸<br/><strong>Async</strong></center></th>
</tr>
</table>

---
layout: default
---

# Practice time!

&nbsp;

Unit 8.1 exercise description: [training.tweede.golf](https://training.tweede.golf/embedded-ecosystem.html)

*Don't forget to* `git pull`!
