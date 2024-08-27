# Unit 8.2 - Portable Rust drivers

<a href="/slides/8_2-portable-drivers/" target="_blank">Slides</a>

## Exercise 8.2.1: LSM303AGR Driver

When you really want to use a device, you want to have a driver. Let's write an actual portable device driver for the accelerometer we've got.

Go to the assignment in `exercises/8-embedded/2-portable-drivers/1-lsm303agr-driver` and implement the `lsm303agr` module.
The goal is to use `embedded-hal` for our hardware definitions, so try not to use any nRF-specific types in that module.

Use the driver to read data from the sensor in `src/main.rs`

*As a bonus exercise, support reading the magnetometer as well*
