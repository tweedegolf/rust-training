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

# Trying it out
Now that you've got the code on your machine, navigate to it using your favorite terminal and run:

```
cd exercises/0-intro
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
