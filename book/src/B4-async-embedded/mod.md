# Module B4: Async on embedded

<a href="/slides/B4-async-embedded.pdf" target="_blank">Slides</a>

Using Embassy, we can run asynchronous Rust code on embedded devices. In this exercise, we'll give Embassy a try.

## B4 Async LIS3DH ID
Just like in exercise B1, we will extract the ID register value from the LIS3DH. This time, though, we'll use Embassy to
do this asyncronously. Open `exercises/B4`, examine the code in `src/main.rs` and run it.

You can find more embassy examples for the nrf52840 here: <https://github.com/embassy-rs/embassy/tree/master/examples/nrf52840/src/bin>

You can try getting the lis3dh to work in embassy with this driver: <https://crates.io/crates/lis3dh-async>
