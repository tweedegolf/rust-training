# Module B1: Embedded basics

<a href="/slides/B1-basics-B2-drivers.pdf" target="_blank">Slides</a>

This first part will give an overview of how the Rust Embedded ecosystem is built up.

We'll then use our newly gained knowledge to get our first application running and read out the ID of the LIS3DH accelerometer.

## B1 LIS3DH ID

To get started we'll setup the i2c on our development kit and read out the ID register of the LIS3DH accelerometer.
The starting point can be found in `exercises/B1-B2/src/main` of this repository.

Try to run the existing project and then fill in the functionality as instructed by the comments.

To use that project, you can use the following commands from inside that folder using the terminal:
- `cargo build`: Builds the project
- `cargo run`: Builds the project, flashes it to the device and listens for any logs which it will display in the terminal. (This uses the `probe-run` tool)

In both cases you can add the `--release` flag to turn on optimizations.

*Note: There is a module called `lis3dh` in the assignment project. This is meant to be used in assignment B2, so it can be ignored for now.*
