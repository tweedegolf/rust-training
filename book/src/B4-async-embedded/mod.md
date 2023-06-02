# Module B4: Async on embedded
Using Embassy, we can run asynchronous Rust code on embedded devices. In this exercise, we'll give Embassy a try.

## B4 Async LIS3DH ID
Just like in exercise B1, we will extract the ID register value from the LIS3DH. This time, though, we'll use Embassy to
do this asyncronously. Open `exercises/B4`, examine the code in `src/main.rs` and run it.

This code requires the use of the nightly compiler.
This should be downloaded automatically due to the `rust-toolchain.toml` file.

You can install and use the toolchain easily by running:
```
rustup toolchain install nightly
cargo +nightly build
```

If you don't want to provide the `+nightly` every time, we can make it the default:

```
rustup default nightly
```

You can find more embassy examples for the nrf52840 here: https://github.com/embassy-rs/embassy/tree/master/examples/nrf52840/src/bin

You can try getting the lis3dh to work in embassy with this driver: https://crates.io/crates/lis3dh-async
