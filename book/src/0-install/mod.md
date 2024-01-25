# Module 0 - Installing the tools

<a href="/slides/0-intro/" target="_blank">Slides</a>

In this file you'll find instructions on how to install the tools we'll use during the workshop.

All of these tools are available for Linux, macOS and Windows users.
We'll need the tools to write and compile our Rust code.
*Important: these instructions are to be followed at home, before the start of the first workshop.*
*If you have any problems with the installation process, please contact us!*

## Rust and Cargo
First we'll need `rustc`, the standard Rust compiler.
`rustc` is generally not invoked directly, but through `cargo`, the Rust package manager.
`rustup` takes care of installing `rustc` and `cargo`.

This part is easy: go to <https://rustup.rs> and follow the instructions.
Please make sure you're installing the latest default toolchain.
Once done, run

```bash
rustc -V && cargo -V
```

The output should be something like this:

```bash
rustc 1.68.2 (9eb3afe9e 2023-03-27)
cargo 1.68.2 (6feb7c9cf 2023-03-26)
```

Using Rustup, you can install Rust toolchains and components. More info:
- <https://rust-lang.github.io/rustup>
- <https://doc.rust-lang.org/cargo>

## Rustfmt and Clippy
To avoid discussions, Rust provides its own formatting tool, Rustfmt.
We'll also be using Clippy, a collection of lints to analyze your code, that catches common mistakes for you.
You'll notice that Rusts Clippy can be a very helpful companion.
Both Rustfmt and Clippy are installed by Rustup by default.

To run Rustfmt on your project, execute:

```bash
cargo fmt
```

To run clippy:

```bash
cargo clippy
```

More info:
- Rustfmt: <https://github.com/rust-lang/rustfmt>
- Clippy: <https://github.com/rust-lang/rust-clippy>

## Visual Studio Code
During the workshop, you can use Visual Studio Code (vscode) to write code in.
Of course, you're free to use your favorite editor, but if you encounter problems, we can't be of very much help.

You can find the installation instructions here: <https://code.visualstudio.com/>.

We will install the Rust-Analyzer plugin as well.
The first one is Rust-Analyzer.
Installation instructions can be found here <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>.
Rust-Analyzer provides a lot of help during development and in indispensable when getting started with Rust.

More info:
- <https://rust-analyzer.github.io/>

## Git
We will use Git as version control tool.
If you haven't installed Git already, you can find instructions here: <https://git-scm.com/book/en/v2/Getting-Started-Installing-Git>.
If you're new to Git, you'll also appreciate GitHubs intro to Git <https://docs.github.com/en/get-started/using-git/about-git> and the Git intro with vscode, which you can find here: <https://www.youtube.com/watch?v=i_23KUAEtUM>.

More info: <https://www.youtube.com/playlist?list=PLg7s6cbtAD15G8lNyoaYDuKZSKyJrgwB->

## Workshop code
Now that everything is installed, you can clone the source code repository.
The repository can be found here: <https://github.com/tweedegolf/rust-workshop>.

Clone the repository. Instructions on cloning the repository can be found here: <https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls>

## Trying it out
Now that you've got the code on your machine, navigate to it using your favorite terminal and run:

```
cd exercises/0-intro/host
cargo run
```

This command may take a while to run the first time, as Cargo will first fetch the crate index from the registry.
It will compile and run the `intro` package, which you can find in `exercises/0-intro/host`.
If everything goes well, you should see some output:

```
   Compiling intro v0.1.0 ([REDACTED]/rust-workshop/exercises/0-intro/host)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/intro`
🦀 Hello, world! 🦀
You've successfully compiled and run your first Rust project!
```

If Rust-Analyzer is set up correctly, you can also click the '▶️ Run'-button that is shown in `exercises/0-intro/host/src/main.rs`.

# Instructions for embedded
This part is relevant only if you're partaking in one of the workshops on embedded Rust.

## Hardware
You should have received the following parts:

- nRF52840-DK
- Breadboard
- LIS3DH Breakout board
- Male-to-male breadboard wires

You'll also need a Micro-USB cable, but we're sure you've got one to spare.

Please check that everything is complete. If not, please contact us.

## Software

Then, we'll install some tools needed to flash the mcu and inspect the code.

Install the `thumbv7em-none-eabihf` toolchain with the following command:
```bash
rustup target add thumbv7em-none-eabihf
```

On `linux` you need to install the "dev" libraries for udev, usb, and ftdi libudev-dev. If you're on Ubuntu:
```bash
# ubuntu
sudo apt install -y libusb-1.0-0-dev libftdi1-dev libudev-dev
```

On `all platforms`:
```bash
rustup component add llvm-tools rustfmt clippy
cargo install probe-run
```

If you're on `linux`, you'll need to update your udev rules.
On ubuntu, run the following inside the workshop folder you just cloned;

```bash
sudo cp 99-jlink-nrf.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
```

If you're on `windows`, we need to install a generic WinUSB driver. You can use [Zadig](https://zadig.akeo.ie/) to select the usb device that uses the jlink driver and install WinUSB on it. 
*This will uninstall the official driver, which means that the official Segger tools will not work anymore after this.* To revert, go to `device manager` and uninstall the usb device. The jlink driver will then be used again for that usb connection.

Then, switch the DK off and on or remove the cable and plug it in again.

## Trying it out
Before we begin, we need to test our hardware. We'll be testing the LIS3DH accelerometer, as well as the nRF52840-DK board. Make sure you have checked out the latest version of the workshop source.

### LIS3DH accelerometer connection
First, let's wire up the LIS3DH accelerometer for I2C.
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
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

*We'll be using other pins later on, but they're not needed to test the hardware*

### Running the test
To test the hardware, please connect the nRF52840-DK to your pc, switch it on, and run
```bash
cd ./exercises/0-intro/embedded
cargo run --release --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.

If not, you may have an accelerometer that uses the alternate i2c address. If so, run this instead:
```bash
cargo run --release --bin test --features alternate-addr
```

## Docs
Datasheets, manuals, and schematics of the parts we are using in the embedded workshops.
### nRF52840
- [nRF52840DK documentation](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html)
- [nRF52840 product specification](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.2.pdf)
### LIS3DH
- [Datsheet](https://www.st.com/resource/en/datasheet/lis3dh.pdf)
- [Schematic (Adafruit)](https://cdn-learn.adafruit.com/assets/assets/000/028/587/original/sensors_sch.png?1447888851)
- [Schematic (SparkFun)](https://cdn.sparkfun.com/datasheets/Sensors/Accelerometers/SparkFun_LIS3DH-Breakout.pdf)
