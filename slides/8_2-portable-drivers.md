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

</br></br>

```c
// TODO: Change to your spi type
#define SPI_TYPE void

void enable_cs(SPI_TYPE* spi) { /* TODO */ }
void disable_cs(SPI_TYPE* spi) { /* TODO */ }

char transfer(SPI_TYPE* spi, char value) {
    // TODO
}

```

::right::

`driver.h`

```c
// Hold spi context
static SPI_TYPE* spi = NULL;
// Init the driver
void init(SPI_TYPE* spi_instance);

char example() {
    enable_cs(spi);
    transfer(spi, 123);
    char result = transfer(spi, 124);
    disable_cs(spi);

    return result;
}
```

---
layout: two-cols
---

https://godbolt.org/z/E3d6MT413

```c
typedef struct Spi {
    unsigned int cs;
    unsigned int write;
    unsigned int read;
} Spi_t;

#define SPI_TYPE Spi_t

static void enable_cs(volatile SPI_TYPE* spi) {
    spi->cs = 1;
}
static void disable_cs(volatile SPI_TYPE* spi) {
    spi->cs = 0;
}
static char transfer(
    volatile SPI_TYPE* spi,
    char value
) {
    spi->write = value;
    return spi->read;
}
```

::right::

```c
char example() {
    enable_cs(spi);
    transfer(spi, 123);
    char result = transfer(spi, 124);
    disable_cs(spi);

    return result;
}
```

```asm
example:
  movs r0, #0
  movs r1, #1
  str r1, [r0]
  movs r1, #4
  movs r2, #123
  str r2, [r1]
  movs r2, #8
  ldr r3, [r2]
  movs r3, #124
  str r3, [r1]
  ldr r1, [r2]
  str r0, [r0]
  uxtb r0, r1
  bx lr
```

---
layout: two-cols
---

# Abstraction in C

Function pointers
- ❌ Inefficient
- ✅ Convenient

```c
typedef void (*EnableCs)();
typedef void (*DisableCs)();
typedef char (*SpiTransfer)(char);

typedef struct Spi {
    EnableCs enable_cs;
    DisableCs disable_cs;
    SpiTransfer spi_transfer;
} Spi_t;

char example(Spi_t spi) {
    spi.enable_cs();
    spi.spi_transfer(123);
    char result = spi.spi_transfer(124);
    spi.disable_cs();
    
    return result;
}
```

::right::

https://godbolt.org/z/d43Tad96j

```asm
example:
  push {r4, r5, r7, lr}
  add r7, sp, #8
  mov r5, r2
  mov r4, r1
  blx r0          ; function call
  movs r0, #123
  blx r5
  movs r0, #124
  blx r5          ; function call
  mov r5, r0
  blx r4          ; function call
  mov r0, r5
  pop {r4, r5, r7, pc}
```
---
layout: two-cols
---

# Abstraction in C

Link-time binding
- ✅ Efficient
- 🟠 Somewhat convenient
- ❌ Error-prone

```c
extern void enable_cs();
extern void disable_cs();
extern char spi_transfer(char value);

char example() {
    enable_cs();
    spi_transfer(123);
    char result = spi_transfer(124);
    disable_cs();

    return result;
}
```

::right::

https://godbolt.org/z/hjesx8qoW

```asm
example:
  ldr r0, .LCPI0_0
  movs r1, #1
  str r1, [r0]
  movs r1, #123
  str r1, [r0, #4]
  ldr r1, [r0, #8]
  movs r1, #124
  str r1, [r0, #4]
  ldr r1, [r0, #8]
  movs r2, #0
  str r2, [r0]
  uxtb r0, r1
  bx lr
.LCPI0_0:
  .long 1000000
```

---
layout: full
---

# Abstraction in Rust

Traits & Generics
- ✅ Reuse traits from `embedded-hal`
- ✅ Efficient
- ✅ Convenient

Reminder (simplified):
```rust
pub trait SpiDevice {
    fn transaction(&mut self, operations: &mut [Operation<'_>]) -> Result<(), ()>;
}

pub enum Operation<'a> {
    Read(&'a mut [u8]),
    Write(&'a [u8]),
    Transfer(&'a mut [u8], &'a [u8]),
    TransferInPlace(&'a mut [u8]),
    DelayNs(u32),
}
```

---
layout: full
---

```rust
pub fn example<Spi: SpiDevice>(spi: &mut Spi) -> u8 {
    let mut buf = [0];

    spi.transaction(&mut [
        Operation::Write(&[123]),
        Operation::Transfer(&mut buf, &[124]),
    ]).unwrap();
    buf[0]
}
```

https://godbolt.org/z/xv33T6e6c

```asm
example::example_instance::h0a336da11b699c51:
  ldr r1, .LCPI1_0
  movs r0, #1
  strb r0, [r1]
  movs r0, #123
  strb r0, [r1, #4]
  movs r0, #124
  strb r0, [r1, #4]
  ldrb r0, [r1, #8]
  movs r2, #0
  strb r2, [r1]
  bx lr
.LCPI1_0:
  .long 1000000
```

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
# Applied

<img src="/images/8_2-embedded-hal.png">

<!--
Draw-io link:

https://viewer.diagrams.net/?tags=%7B%7D&lightbox=1&highlight=0000ff&edit=_blank&layers=1&nav=1&title=Untitled%20Diagram.drawio#R%3Cmxfile%3E%3Cdiagram%20name%3D%22Page-1%22%20id%3D%22ISt-d_c7fnAXG8RkIMt1%22%3E7VjbbuIwEP0aHluFmITwWKDdVmIlKlba7qOJJ4m3Jo4cU6Bfv3ZiE3Lp0qrAqtI%2BMT5zsX1m7HHooclq%2B03gLPnOCbCe65BtD017rtsfIEf9aGRXIkMflUAsKDFGFbCgr2BA4xevKYG8Zig5Z5JmdTDkaQqhrGFYCL6pm0Wc1WfNcAwtYBFi1kZ%2FUiKTEg08p8LvgcaJnbnvGM0Sh8%2Bx4OvUzJfyFErNCtswxjRPMOGbAwjd9tBEcC5LabWdANO0WsZKv7s3tPslC0jlexye4H70%2BDhyHwczmv64e96%2BMnllorxgtjZU3N%2FMzHrlztJTbBB0HKeHxpuESlhkONTajSoIhSVyxdSor0TMaJwqmUGk1jXOlSFNYwW42jmijE0446KIjAiGIAq1mRT8GQ40fhjAMlIas0IQErZvbr2%2FJ1TVKPAVSLFTJtbB5sCUpzs0402VbNemNDlMdGBAbAos3seumFaCIfsDxPst4uc3kwsSH0WRG3YST%2Fyl7%2FknIj5oEB%2B0ie%2BjLuL9cxHvtohfzB8%2BR3yzpj0IyKCL2sBdIt8%2FmqoTEO8OGsSPOoh3O4g%2FG%2B%2BoxfuD%2B8mC%2FwK8o46b5rK89wct4nuuz%2FT2CX1RYqxFWC2BKOKvEsysWk13YNHhJAWmUoVeZHQKL1RlydgsRdOrGauRdtUbMy0SLPFCclG06%2BPp34J9S3SUA%2FRVQQy7ymHkDxE%2B0Q2HnHcctMElE26r6yDhNjsOEVTt9jRtRpR7GTO8BDbnOZWUazxUzKk50FjzStUba9YwWFFC9Mx7gxsTca840rqCELpb1zLwBt6JTrLXfDN0tK7ON8PZngxu%2Bwr9f5JPd5IHzbfKvz%2FJ3tkT%2FuCGn051LkHnMwNB1b710S%2BgeTX%2BEvlv3uSdrfuy%2BfdbnANR361myIVMeMxTzG4rdFy%2FyyubGeeZofc3SLkzdOO15PVkwJbKJ%2BOu5V9avvbMaLo9UE13dpCq%2FRZO18FwZAHj6QwtUDkXo5r3QbkUYC6xkDf62173FIbznIYWvqPMrrYsCfvRjopoxLoVX%2BMFYjyq3qCJ%2FHuhKN75WoTwjhtZrSkGefQR1i49AQxL1ZJr036gjtSw%2BgOh0B38QYNu%2FwA%3D%3C%2Fdiagram%3E%3C%2Fmxfile%3E
-->

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
layout: full
---

# Low level driver in C

- Enum & `#define` for definitions
- Functions for read/write

```c
enum Reg
{
    REG_ID = 0x01,
    REG_IER = 0x02,
    REG_IDR = 0x03,
    REG_ISR = 0x04,
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

# Low level driver in Rust

- Enum & `const` for definitions
- Functions for read/write based on shared trait

```rust
use embedded_hal::spi::SpiDevice;

#[repr(u8)]
pub enum Register {
    Id = 0x01,
    Ier = 0x02,
    Idr = 0x03,
    Isr = 0x04,
}

const ID_MANUFACTURER_MASK: u8 = 0xF0;
const ID_MANUFACTURER_POS: u8 = 4;

pub fn read_register<Spi: SpiDevice>(spi: &mut Spi, address: Register) -> u8 {
    let mut buffer = [address as u8, 0];
    spi.transfer_in_place(&mut buffer).unwrap();
    buffer[1]
}
```

---
layout: full
---

# Low level driver in Rust

- Ideomatic: Device driver is a struct

```rust
pub struct Device<SPI: SpiDevice> {
    interface: SPI,
}

impl<SPI: SpiDevice> Device<SPI> {
    pub fn new(interface: SPI) -> Self {
        Self { interface }
    }

    pub fn read_register<Spi: SpiDevice>(&mut self, address: Register) -> u8 {
        let mut buffer = [address as u8, 0];
        self.interface.transfer_in_place(&mut buffer).unwrap();
        buffer[1]
    }
}

fn foo() {
    let interface = // Init interface
    let mut my_device = Device::new(interface);
    let value = my_device.read_register(Register::Id);
}
```

---
layout: two-cols
---

# Possibilities in Rust

Add typestate to high-level driver

- Allows compiler to check valid use of driver

Usage:
```rust
let mut device: Device<_, Idle> =
    Device::new(/* spi interface */);

device.wait_ready();
//     ^^^^^^^^^^ Compile error
// `wait_ready` not implemented for `Device<_, Idle>`
```

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
    pub fn wait_ready(self) 
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
- `embedded-io`
- `embedded-storage`
- `embedded-graphics`
- `embedded-nal`
- `accelerometer`
- `usb-device`
- `radio`

---
layout: with-footer
---

# Practice time!

&nbsp;

Unit 8.2 exercise description: [training.tweede.golf](https://training.tweede.golf/portable-drivers.html)

*Don't forget to* `git pull`!
