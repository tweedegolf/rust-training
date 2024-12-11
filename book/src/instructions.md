# Unit 1.2 - Introduction

## Exercise 1.2.1: Setup Your Installation

In this file you'll find instructions on how to install the tools we'll use during the course.

All of these tools are available for Linux, macOS and Windows users.
We'll need the tools to write and compile our Rust code, and allow for remote mentoring.
*Important: these instructions are to be followed at home, before the start of the first tutorial.*
*If you have any problems with installation, contact the lecturers! We won't be addressing installation problems during the first tutorial.*

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
rustc 1.79.0 (129f3b996 2024-06-10)
cargo 1.79.0 (ffa9cf99a 2024-06-03)
```

Using Rustup, you can install Rust toolchains and components. More info: 
- <https://rust-lang.github.io/rustup>
- <https://doc.rust-lang.org/cargo>

## Rustfmt and Clippy
To avoid discussions, Rust provides its own formatting tool, Rustfmt.
We'll also be using Clippy, a collection of lints to analyze your code, that catches common mistakes for you.
You'll find that Rusts Clippy can be a very helpful companion.
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
During the course, we will use Visual Studio Code (vscode) to write code in.
Of course, you're free to use your favorite editor, but if you encounter problems, you can't rely on support from us.
Also, we'll use VSCode to allow for remote collaboration and mentoring during remote training sessions.

You can find the installation instructions here: <https://code.visualstudio.com/>.

We will install some plugins as well.
The first one is Rust-Analyzer.
Installation instructions can be found here <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>.
Rust-Analyzer provides a lot of help during development and in indispensable when getting started with Rust.

Another plugin we'll use is CodeLLDB.
This plugin enables debugging Rust code from within vscode.
You can find instructions here: <https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb>.

If you're following the training remotely, install the Live Share plugin as well.
We will use the plugin to share code and provide help during remote tutorial sessions.
Installation instructions can be found here: <https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare>

More info:
- <https://rust-analyzer.github.io/>
- <https://code.visualstudio.com/learn/collaboration/live-share>

### Tip

This repo contains quite a lot of rust projects and due to the complicated setup of the repo Rust Analyzer can't autodiscover them well.

To fix this we've specified the projects manually in the `.vscode/settings.json` file. To reduce the burden on your computer, you can comment out any of the projects that we're not using in our training.

## Git
During the trainings, you'll need the Git version control tool.
If you haven't installed Git already, you can find instructions here: <https://git-scm.com/book/en/v2/Getting-Started-Installing-Git>.
If you're new to Git, you'll also appreciate GitHubs intro to Git <https://docs.github.com/en/get-started/using-git/about-git> and the Git intro with vscode, which you can find here: <https://www.youtube.com/watch?v=i_23KUAEtUM>.

More info: <https://www.youtube.com/playlist?list=PLg7s6cbtAD15G8lNyoaYDuKZSKyJrgwB->

## Course code
Now that everything is installed, you can clone the source code repository using Git.
The repository can be found here: <https://github.com/tweedegolf/rust-training>.

Instructions on cloning the repository can be found here: <https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls>

### Trying it out
Now that you've got the code on your machine, navigate to it using your favorite terminal and run:

```
cd exercises/1-course-introduction/1-introduction/1-setup-your-installation
cargo run
```

This command may take a while to run the first time, as Cargo will first fetch the crate index from the registry.
It will compile and run the `intro` package, which you can find in `exercises/1-course-introduction/1-introduction/1-setup-your-installation`.
If everything goes well, you should see some output:

```
   Compiling intro v0.1.0 ([/path/to/rust-workshop]/exercises/1-course-introduction/1-introduction/1-setup-your-installation)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/intro`
🦀 Hello, world! 🦀
You've successfully compiled and run your first Rust project!
X: 2; Y: 2
```
If Rust-Analyzer is set up correctly, you can also click the '▶️ Run'-button that is shown in `exercises/1-course-introduction/1-introduction/1-setup-your-installation/src/main.rs`.
With CodeLLDB installed correctly, you can also start a debug session by clicking 'Debug', right next to the '▶️ Run'-button.
Play a little with setting breakpoints by clicking on a line number, making a red circle appear and stepping over/into/out of functions using the controls.
You can view variable values by hovering over them while execution is paused, or by expanding the 'Local' view under 'Variables' in the left panel during a debug session.

# Instructions for FFI module
*This part is relevant only if you're partaking in one of the modules on Rust FFI.*

For doing FFI we will need to compile some C code and for that we need a C compiler installed.
We've chosen to use `clang` in our excercises.

The prerequisite is that calling `clang` in your terminal should work. If it doesn't, follow the instructions for your platform below.

## Linux

For the bookworms using a Debian-like:
```bash
sudo apt update
sudo apt install clang
```

If you're on Arch, btw:
```bash
sudo pacman -S clang
```

For those tipping their Fedora's:
```bash
sudo dnf install clang
```

## Windows

Always make sure to select the option to add the install to `path`.

Using winget:
```ps
winget install -i -e --id LLVM.LLVM
```

For the sweethearts using chocolatey:
```ps
choco install llvm
```

For the handsome people preferring manual installation:
- Go to the releases page: https://github.com/llvm/llvm-project/releases
- Go to a recent release
- Search for LLVM-[VERSION]-win64.exe and download it
- Run the exe

## MacOS

Using brew:
```bash
brew install llvm
```

Then also add to path, e.g.:
```bash
echo 'export PATH="$(brew --prefix llvm)/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

# Instructions for embedded
*This part is relevant only if you're partaking in one of the modules on embedded Rust.*

## Hardware
We will use the [BBC micro:bit](https://microbit.org/buy/bbc-microbit-single) V2 and either you've already got it or we will bring it with us.

You'll also need a Micro-USB cable, but we're sure you've got one to spare.

Please check that everything is complete. If not, please contact us.

## Software

Then, we'll install some tools needed to flash the mcu and inspect the code.

Install the `thumbv7em-none-eabihf` toolchain with the following command:
```bash
rustup target add thumbv7em-none-eabihf
```

We'll also install a couple of tools that let us inspect our binaries:

```bash
rustup component add llvm-tools
cargo install cargo-binutils
```

Now, let's install [probe-rs](https://probe.rs). Follow the [installation instructions](https://probe.rs/docs/getting-started/installation/). Probe-rs talks with the debug interface on the micro:bit, to flash your application, log messages, or even set breakpoints and read out device memory.

If you're on `linux`, you'll need to update your udev rules.
On ubuntu or fedora, run the following inside the workshop folder you just cloned;

```bash
sudo cp 99-microbit-v2.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
sudo udevadm trigger
```

It's possible that probe-rs detects two debugging interfaces. This is known to happen on Windows.
In that case, go to the `.cargo/config.toml` and change the runner to the one that specifies the exact probe
it needs to use. Make sure the values are the same as what probe-rs reports is one of the interfaces.
Unsure? You can run `probe-rs list` to get a list of all connected probes.

## Trying it out
Before we begin, we need to test our hardware. We'll be testing the nRF52833 microcontroller and the LSM303AGR motion sensor, that are present on the micro:bit V2. Make sure you have checked out the latest version of the workshop source.

### Running the test
To test the hardware, please connect the micro:bit V2 to your pc, switch it on, and run
```bash
cd ./exercises/1-course-introduction/1-introduction/2-embedded
cargo run --release
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.

## Docs
Datasheets, manuals, and schematics of the parts we are using in the embedded workshops.
### BBC micro:bit V2
- [Hardware doc](https://tech.microbit.org/hardware/schematic/)
- [Schematic](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF)
### nRF52833
- [nRF52833 product specification](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.6.pdf)
### LSM303AGR
- [Datsheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf)
