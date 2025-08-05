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
routerMode: hash
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
        <center><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" width="75" height="75"></center>
        <center>
            <img style="display:inline-block" src="https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg" width="75" height="75">
            <img style="display:inline-block" src="https://upload.wikimedia.org/wikipedia/commons/1/19/C_Logo.png" width="75" height="75">
        </center>
    </td>
    <td></td>
    <td><center><img src="https://doc.rust-lang.org/1.85.0/cargo/images/Cargo-Logo-Small.png" width="75" height="75"></center></td>
    <td><center><img src="https://raw.githubusercontent.com/probe-rs/webpage/044a6bc2d87cecb1784d0def1f460a3dfbc062eb/src/images/banner.svg" width="75" height="75"></center></td>
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
dp.WDT.CONFIG.modify(|_, w| w.per()._16());
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
//! This example shows how to use UART (Universal asynchronous receiver-transmitter) in the RP2040 chip.

#![no_std]
#![no_main]

use embassy_rp::uart;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Init the hal
    let p = embassy_rp::init(Default::default());

    // Init the uart
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_with_rtscts_blocking(p.UART0, p.PIN_0, p.PIN_1, p.PIN_3, p.PIN_2, config);

    loop {
        uart.blocking_write("hello there!\r\n".as_bytes()).unwrap();
        cortex_m::asm::delay(1_000_000);
    }
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

### SPI trait:
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

# Runtimes

<br/><br/><br/>

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
