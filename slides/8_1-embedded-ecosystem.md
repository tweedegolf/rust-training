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
    <td colspan="6"><center><br/>Driver</center></td>
</tr>
<tr class="hide">
    <td colspan="6"><center><br/><pre>embedded-hal</pre></center></td>
</tr>

<tr class="hide">
    <td colspan="1"><center><br/>HAL<br/><pre>atsamd-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>embassy-nrf</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>embassy-stm32</pre></center></td>
    <td colspan="1"><center><br/>HAL<br/><pre>rp-hal</pre></center></td>
</tr>

<tr>
    <td><center><br/>PAC<br/><pre>SAMD21E</pre></center></td>
    <td><center><br/>PAC<br/><pre>nRF52833</pre></center></td>
    <td><center><br/>PAC<br/><pre>nRF9160</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32H743</pre></center></td>
    <td><center><br/>PAC<br/><pre>STM32L476</pre></center></td>
    <td><center><br/>PAC<br/><pre>RP2040</pre></center></td>
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
hal::bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    // Init RTT control block
    rtt_init_print!();

    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use ``dp` to get a handle to the peripherals
    let dp = hal::init(Default::default());

    rprintln!("Starting");

    let config = twim::Config::default();
    let mut twim0 = Twim::new(dp.TWISPI0, Irqs, dp.P0_03, dp.P0_04, config);

    rprintln!("Reading...");

    let mut buf = [0u8; 16];
    twim0.blocking_write_read(0xAB, &mut [0x00], &mut buf).unwrap();

    rprintln!("Read: {:02x?}", buf);
    exit();
}
```

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr class="hide">
    <td colspan="6"><center><br/>Driver</center></td>
</tr>
<tr class="hide">
    <td colspan="6"><center><br/><pre>embedded-hal</pre></center></td>
</tr>

<tr>
    <td colspan="1"><center><br/>HAL<br/><pre>atsamd-hal</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>embassy-nrf</pre></center></td>
    <td colspan="2"><center><br/>HAL<br/><pre>embassy-stm32</pre></center></td>
    <td colspan="1"><center><br/>HAL<br/><pre>rp-hal</pre></center></td>
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
# `embedded-hal`

The glue of the entire ecosytem
- Contains abstractions for many common operations

<br/>

### SPI example trait:
```rust
pub trait SpiDevice<Word: Copy + 'static = u8>: ErrorType {
    fn transaction(&mut self, operations: &mut [Operation<'_, Word>]) -> Result<(), Self::Error>;
}

pub enum Operation<'a, Word: 'static> {
    Read(&'a mut [Word]),
    Write(&'a [Word]),
    Transfer(&'a mut [Word], &'a [Word]),
    TransferInPlace(&'a mut [Word]),
    DelayNs(u32),
}
```

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr class="hide">
    <td colspan="6"><center><br/>Driver</center></td>
</tr>
<tr>
    <td colspan="6"><center><br/><pre>embedded-hal</pre></center></td>
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

# Typestate

State encoded in the *type* of the variable

```rust
// https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/blinky.rs

use nrf52833_hal::gpio::{Pin, p0::P0_04, Input, PullDown, Output, PushPull};

/// Take an nRF pin.
/// It must be:
/// - Port 0 pin 4 (Compile time constant)
/// - Configured as input
fn read_status(pin: Input<'_, P0_04>) -> bool {}

/// Take an nRF pin.
/// It must be:
/// - Any port and pin (Runtime variable)
/// - Configured as output
fn set_led_level(pin: Output<'_, AnyPin>, enabled: bool) {}
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
