---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 8.4 the Embassy framework"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: "Rust - 8.4 the Embassy framework"
routerMode: hash
---

# Rust programming

Module 8: Rust for embedded

## Unit 4

The Embassy Framework

---
layout: with-footer
---

# So what _is_ embassy?

A framework containing
- Executor: [`embassy-executor`](https://docs.rs/embassy-executor/latest/embassy_executor/)
- [`embedded-hal-async`](https://docs.rs/embedded-hal-async/latest/embedded_hal_async/)-based HALs:
  - [`embassy-nrf`](https://docs.embassy.dev/embassy-nrf/git/nrf52833/index.html)
  - [`embassy-stm32`](https://docs.embassy.dev/embassy-stm32/)
  - ...
- Utils: [`embassy-sync`](https://docs.rs/embassy-sync/latest/embassy_sync/), [`embassy-time`](https://docs.rs/embassy-time/latest/embassy_time/), ...
- Bootloader: [`embassy-boot`](https://docs.rs/embassy-boot/latest/embassy_boot/)
- USB: [`embassy-usb`](https://docs.rs/embassy-usb/latest/embassy_usb/)
- Bluetooth: [`nrf-softdevice`](https://github.com/embassy-rs/nrf-softdevice),  [`trouBLE`](https://github.com/embassy-rs/trouble) (experimental)

---
layout: with-footer
---

# Some notes about embassy

- Very much in flux
  - use [docs.embassy.dev](https://docs.embassy.dev/) for latest git version docs
  - or run `cargo doc --open` in your project
- Uses [`chiptool`](https://github.com/embassy-rs/chiptool) instead of `svd2rust` for PACs (experimental)
- [Multi-priority using multiple executors](https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/multiprio.rs)

---
layout: with-footer
---

# Practice time!

&nbsp;

Unit 8.4 exercise description: [training.tweede.golf](https://training.tweede.golf/embassy-framework.html)

*Don't forget to* `git pull`!
