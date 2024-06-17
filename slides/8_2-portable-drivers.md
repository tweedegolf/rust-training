---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 8.2: Portable Rust drivers"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 8.2: Portable Rust drivers"
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

## Unit 2

Portable Rust drivers

---
layout: with-footer
---

# Portable Rust drivers

- We want drivers for devices
- We want them to be portable

**Contents**
- How does C do it?
- How does Rust compare to that?
- Low level & high level drivers
- Example low level driver
- Possibilities in Rust


---
layout: two-cols
---

# Abstraction in C

Fill-in functions
- ✅ Efficient
- ❌ Inconvenient

::right::

```c
#include <stddef.h>

// TODO Change to your spi type
#define SPI_TYPE void

static SPI_TYPE* spi = NULL;
void init(SPI_TYPE* spi_instance);

static void enable_cs(SPI_TYPE* spi) {/* Todo */ }

static void disable_cs(SPI_TYPE* spi){/* Todo */ }

static char transfer(SPI_TYPE* spim, char value)
{
    // Todo: Transfer 'value' over the bus and return the response
}

char example() {
    enable_cs(spi);
    transfer(spi, 0xDE);
    char result = transfer(spi, 0xAD);
    disable_cs(spi);

    return result;
}
```

---
layout: full
---

<img src="/images/8_2-c-fill-in-light.png">

---
layout: two-cols
---

# Abstraction in C

Function pointers
- ❌ Inefficient
- ✅ Convenient

::right::

```c
#include <stddef.h>

typedef void (*EnableCs)();
typedef void (*DisableCs)();
typedef char (*SpiTransfer)(char);
static EnableCs enable_cs = NULL;
static DisableCs disable_cs = NULL;
static SpiTransfer spi_transfer = NULL;

void init(
    EnableCs init_enable,
    DisableCs init_disable,
    SpiTransfer init_transfer,
) {
    enable_cs = init_enable;
    disable_cs = init_disable;
    spi_transfer = init_transfer;
}
char example() {
    enable_cs();
    spi_transfer(0xDE);
    char result = spi_transfer(0xAD);
    disable_cs();
    return result;
}
```

---
layout: full
---

<img src="/images/8_2-c-function-pointers-light.png"/>

---
layout: two-cols
---

# Abstraction in C

Link-time binding
- ✅ Efficient
- 🟠 Somewhat convenient
- ❌ Error-prone

::right::

```c
#include <stddef.h>

extern void enable_cs();
extern void disable_cs();
extern char spi_transfer(char value);

char example() {
    enable_cs();
    spi_transfer(0xDE);
    char result = spi_transfer(0xAD);
    disable_cs();
    return result;
}
```

---
layout: full
---

<img src="/images/8_2-c-link-binding-light.png"/>

---
layout: two-cols
---

# Abstraction in Rust

Traits & Generics
- ✅ Reuse traits from `embedded-hal`
- ✅ Efficient
- ✅ Convenient

::right::

```rust
/// My fictional SPI peripheral
struct SpiPeripheral;

impl SpiPeripheral {
    #[inline]
    fn enable_cs(&mut self) {
        unsafe { ptr::write_volatile(0x2000000 as *mut u8, 0) }
    }

    #[inline]
    fn disable_cs(&mut self) {
        unsafe { ptr::write_volatile(0x2000000 as *mut u8, 1) }
    }
}

impl spi::ErrorType for SpiPeripheral {
    // SPI operations never fail on this peripheral
    type Error = core::convert::Infallible;
}
```

---
layout: full
---

```rust
impl SpiDevice for SpiPeripheral {
    #[inline]
    fn transaction(&mut self, operations: &mut [spi::Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.enable_cs();
        for op in operations.iter_mut() {
            match op {
                spi::Operation::Write(w) => unsafe {
                    w.into_iter()
                        .for_each(|word| ptr::write_volatile(0x2000001 as *mut u8, *word))
                },
                spi::Operation::Transfer(r, w) => unsafe {
                    let len = w.len().max(r.len());
                    let it = w.into_iter().chain(repeat(&0x00));
                    let it = it
                        .zip(r.into_iter().map(Some).chain(repeat_with(|| None))).take(len);
                    it.for_each(|(w, r)| {
                        ptr::write_volatile(0x2000001 as *mut u8, *w);
                        r.map(|r| *r = ptr::read_volatile(0x2000001 as *mut u8));
                    });
                },
                _ => todo!(),
            }
        }
        self.disable_cs();
        Ok(())
    }
}
```

---
layout: full
---

<img src="/images/8_2-rust-light.png"/>

---
layout: with-footer
---

# Overview

<table class="emb-overview">
<tr>
    <td colspan="7"><center>Driver<br/>⬇️</center></td>
</tr>
<tr>
    <td colspan="7"><center><pre>embedded-hal</pre></center></td>
</tr>

<tr>
    <td colspan="1"><center>⬆️<br/>HAL<br/><pre>atsamd</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>nrf-hal</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>stm32h7xx-hal</pre></center></td>
    <td colspan="2"><center>⬆️<br/>HAL<br/><pre>stm32l4xx-hal</pre></center></td>
</tr>

<tr>
    <td><center>⬇️<br/>PAC<br/><pre>nRF52833</pre></center></td>
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

# Low level & high level drivers

**Low level driver**
- Hardware abstractions + reading/writing registers
- Register/buffer/command definitions

**High level driver**
- Implement common routines
- Requirement assertions
- Common interface export

---
layout: two-cols
---

# Low level driver in C

- Enum & `#define` for definitions
- Functions for read/write

::right::

```c
enum Reg
{
    REG_ID = 0x01,
    REG_IER = 0x02,
    REG_IDR = 0x03,
    REG_ISR = 0x04,
    REG_PIN = 0x05,
    REG_PORT = 0x06,
};

#define ID_MANUFACTURER_MASK 0xF0
#define ID_MANUFACTURER_POS 4

static char read_register(enum Reg address)
{
    enable_cs();
    transfer(address);
    char result = transfer(0x00);
    disable_cs();
    return result;
}
```

---
layout: full
---

<img src="/images/8_2-low-level-c-driver-light.png"/>

---
layout: two-cols
---

# Low level driver in Rust

- Enum for definitions
- Device struct represents chip instance
- Functions for read/write

::right::

```rust
use embedded_hal::spi::SpiDevice;

    #[repr(u8)]
    pub enum Register {
        Id = 0x01,
        Ier = 0x02,
        Idr = 0x03,
        Isr = 0x04,
        Pin = 0x05,
        Port = 0x06,
    }

    pub struct Device<SPI: SpiDevice> {
        spi: SPI,
    }

    impl<SPI: SpiDevice> Device<SPI> {
        pub fn new(spi: SPI) -> Self {
            Self { spi }
        }
    }
```

---
layout: full
---

<img src="/images/8_2-low-level-rust-driver-light.png"/>

---
layout: with-footer
---

# Possibilities in Rust

Much more is possible:

```rust
pub fn example() {
    let mut device = Device::new(Spi);

    let manufacturer = device.id().read().manufacturere();

    if manufacturer != 0 {
        device.port().modify(|_, w| w.enable_7(true));
    }
}
```

```c
void example() {
    char manufacturer = (read_register(REG_ID) & ID_MANUFACTURER_MASK) >> ID_MANUFACTURER_POS;

    if (manufacturer != 0) {
        char reg = read_register(REG_PORT);
        reg |= 0x80;
        write_register(REG_PORT, reg);
    }
}
```

---
layout: two-cols
---

# Possibilities in Rust

Add typestate to high-level driver

- Allows compiler to check valid use of driver

::right::

```rust
struct Idle;
struct Sending;

pub struct Device<SPI, STATE> {
    spi: SPI,
    _state: PhantomData<STATE>,
}


impl<SPI: SpiDevice> Device<SPI, Idle> {
    pub fn send(self, data: &[u8]) ->
        Device<SPI, Sending> 
    {
        todo!("Start sending, go to Sending state")
    }
}

impl<SPI: SpiDevice> Device<SPI, Sending> {
    pub async fn wait_ready(self) 
        -> Device<SPI, Ready> {
        todo!("Block until done, go to Idle state")
    }
}
```

---
layout: with-footer
---

# Possibilities in Rust

Much more is possible:
- `embedded-hal`
- `radio`
- `embedded-nal`
- `usb-device`
- `embedded-graphics`
- `accelerometer`
- `embedded-storage`

---
layout: with-footer
---

# Practice time!

&nbsp;

Unit 8.2 exercise description: [training.tweede.golf](https://training.tweede.golf/portable-drivers.html)

*Don't forget to* `git pull`!
