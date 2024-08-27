# Unit 8.3 - Async on Embedded

<a href="/slides/8_3-async-on-embedded/" target="_blank">Slides</a>

## Exercise 8.3.1: Compass
In this exercise, we'll use the [`lsm303agr`](https://docs.rs/lsm303agr) driver. Although the documentation doesn't show it, it supports async if you enable its `async` [feature](https://doc.rust-lang.org/cargo/reference/features.html)

Have a look at the examples in the [`lsm303agr-rs` repository](https://github.com/eldruin/lsm303agr-rs) to get an idea of how to use this driver.

Using the `lsm303agr` driver, implement a compass. You can use the `dial` module to indicate the north pole's direction. You'll find a couple of [`todo!()`](https://doc.rust-lang.org/std/macro.todo.html)'s with instructions.

*Compiling the starter code yields a bunch of warnings. They'll be gone once your done.*

## Exercise 8.3.2: Blinky compass
The [channel sender - receiver example](https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/channel_sender_receiver.rs) in the embassy repository shows how to spawn separate tasks, and how to use channels to communicate between tasks. Using that knowledge, make the indicator LED in the `dial` module blink while magnetometer measurements are taken at the same time.

As we're not using `defmt` in this exercise, replace the `unwrap!(<expr>)` macro invocations with `<expr>.unwrap()` calls.
