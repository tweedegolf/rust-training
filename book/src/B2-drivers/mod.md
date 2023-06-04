# Module B2: Cross-platform drivers

<a href="/slides/B1-basics-B2-drivers.pdf" target="_blank">Slides</a>

When you really want to use a device, you want to have a driver. We are going to learn about those next.

## B2 LIS3DH Driver
Let's write an actual portable device driver for the accelerometer we've got.

Got to the assignment in `./exercises/B1-B2` and implement the `lis3dh` module.
The goal is to use `embedded-hal` for our hardware definitions, so try not to use any nrf specific types in that module.

You should have all the information you need in the previous chapters of this book, but please do ask questions if you have any.
