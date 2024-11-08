# Unit 8.1 - The Rust embedded ecosystem

<a href="/slides/8_1-embedded-ecosystem/" target="_blank">Slides</a>

## Exercise 8.1.1: LSM303AGR ID
Use our newly gained knowledge to get our first application running and read out the ID of the LSM303AGR accelerometer.
We can communicate with the LSM303AGR using the I2C that is present on the micro:bit board. Note that the nRF52833 supports I2C with its TWIM (Two-Wire Interface Master) peripheral.

To get started we'll setup the i2c peripheral on our development kit and read out the ID register of the LSM303AGR accelerometer.
The starting point can be found in the file at `exercises/8-embedded/1-embedded-ecosystem/src/main.rs` in the [repository](https://github.com/tweedegolf/rust-training).

Try to run the existing project and then fill in the functionality as instructed by the comments.

To use that project, you can use the following commands from inside that folder using the terminal:
- `cargo build`: Builds the project
- `cargo run`: Builds the project, flashes it to the device and listens for any logs which it will display in the terminal. (This uses the `probe-rs run` tool)

In both cases you can add the `--release` flag to turn on optimizations.

<details>
    <summary><b>Some pointers to help you get started</b></summary>

- You can find the documentation on the HAL here on [docs.embassy.dev](https://docs.embassy.dev/embassy-nrf/git/nrf52833/index.html). This website houses the docs for embassy for every available chip. Normally you'd search at [docs.rs](https://docs.rs), but that only shows one possible configuration of the HAL.
- To find out how to configure I2C for the nRF52833: [embassy-nrf TWIM demo example](https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/twim.rs).
- You can find the LSM303AGR data sheet here: <https://www.st.com/resource/en/datasheet/lsm303agr.pdf>. You can find the accelerometer device ID in the `WHO_AM_I_A` register, at register address `0x0F`. You'll need `0x19` to address the accelerometer itself.
- Use the [`Twim::blocking_write_then_read`](https://docs.embassy.dev/embassy-nrf/git/nrf52833/twim/struct.Twim.html#method.blocking_write_read) method to first write the device address, then write the register address, and then read its contents into a buffer.
</details>
