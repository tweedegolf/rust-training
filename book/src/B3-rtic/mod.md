
# Module B3: RTIC

<a href="/slides/B3-rtic.pdf" target="_blank">Slides</a>

The goal of this exercise is to get acquainted with RTIC. Use your experience (and code) from parts B1 and B2.

## B3 Simple RTIC app

### LIS3DH accelerometer connection
First, let's wire up the LIS3DH accelerometer for I2C usage. We'll connect the LIS3DH INT1 pin to the nRF's P0.02.
**Please turn off your DK**. Then, wire up the accelerometer, referring to the table below.

| LIS3DH Pin | nRF52 pin 	  |
|------------|----------------|
| VIN (+)    | VDD            |
| 3vo        | -              |
| GND (-)    | GND            |
| SCL        | P0.27          |
| SDA        | P0.26          |
| SDO        | -              |
| CS'        | -              |
| INT1       | P0.02          |
| A1         | -              |
| A2         | -              |
| A3         | -              |


### Instructions
1. For this exercise, we'll be working in `exercises/B3`. Inside the `src` folder, you'll find a couple of files:
    - `lib.rs` Where all modules are declared. No need to edit
    - `hal_import.rs` HAL compatibility module. You can leave it as is.
    - `acc.rs` contains a `config_acc` function. This function can be used to configure the LIS3DH to raise an interrupt if it experiences acceleration above 1.1g. It uses the `lis3dh` driver crate in order to do so. All you need to do is pass it a TWIM instance.
    - `main.rs` Here's where your magic happens. This is a typical RTIC application. It contains several examples of topics we just covered in the talk. You'll find the instructions at the bottom of the `init` task.

2. Follow the instructions in `main.rs`.
3. If you're done early, try to get some cool LED animation sequence going using task scheduling.

### Resources
- [The RTIC book](https://rtic.rs)
- [nRF-HAL examples](https://github.com/nrf-rs/nrf-hal/tree/master/examples)
- [LIS3DH driver repository](https://github.com/BenBergman/lis3dh-rs)
- [LIS3DH driver documentation](https://docs.rs/lis3dh/latest/lis3dh/)
