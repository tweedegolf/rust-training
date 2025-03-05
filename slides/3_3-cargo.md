---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 3.3: Cargo Tooling"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 3.3: Cargo Tooling"
routerMode: hash
---

# Rust programming

Module 3: Crate Engineering

## Unit 3

Cargo Tooling and Managing Dependencies

---
layout: default
---

# Managing a project with cargo:

The Basics:

- `cargo new <PROJECT>`
- `cargo init` (in an existing directory)
- `cargo run` (runs `main.rs`)
- `cargo test` (runs tests in `lib.rs`)

---
layout: default
---

# Managing a project with cargo:

The Basics:

- `cargo new <PROJECT>`
- `cargo init` (in an existing directory)
- `cargo run` (runs `main.rs`)
- `cargo test` (runs tests in `lib.rs`)

Dependencies:

- `cargo add <DEPENDENCY>`
- `cargo update`

All these subcommands have plenty of options!

---
layout: default
---

# Code tools accessible via cargo

Even in small projects, use:

- `cargo fmt`
- `cargo clippy`

Run clippy often --- every update has new lints!

---
layout: default
---

# Look at your `Cargo.toml`

Dependencies can be managed in `Cargo.toml`

- "Tom's Obvious Minimal Language"

- Most crates have features that can be enabled!
  - `cargo add --feature`
  - Example: https://docs.rs/crate/rustls/latest/features

- You can also define features for your own project:

```toml
[dependencies]
colored = { version="3.0.0", optional = true }

[features]
dev-mode = []
use-fancy-colors = ["dep:colored"]
```

---
layout: default
---

# Conditional compilation (1)

Testing for features in your code is easy:

- Feature attribute applies to the next statement
```rust
#[cfg(feature = "dev-mode")]
fn emit_debug_noise(volume: u8) {
    // ...
}

fn main() {
    println!("Welcome!");
    #[cfg(feature = "dev-mode")]
    emit_debug_noise(5);
    // ...
    #[cfg(feature = "dev-mode")]
    emit_debug_noise(0);
    println!("Bye!");
}
```

---
layout: default
---

# Conditional compilation (2)

Testing for features in your code is easy:

- `cfg!` expressions
```rust
fn main() {
    println!("Welcome!");
    if cfg!(feature = "fancy-colors") {
        // ...
    } else {
        // ...
    }
}
```

---
layout: default
---

# Conditional compilation (3)

You can also test for other things:

- `cfg!` expressions
```rust
fn main() {
    if cfg!(target_os = "macos") {
        println!("Compiled for MacOS");
    } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
        println!("Compiled for GNU or UNIX");
    } else {
        compile_error!("please use a serious OS");
    }
}
```

---
layout: default
---

# How to select a dependency

#### The Algorithm:

1. Go to https://lib.rs or https://crates.io and search
2. Evaluate the https://docs.rs page for the crate
3. Evaluate the liveness of the crate::
   - How many downloads?
   - How many reverse dependencies?
   - Last release when?

#### If absolutely critical:

4. Evaluate the (GitHub) repository
5. Consult `cargo vet` or `cargo crev`
6. Look at the source code

---
layout: default
---

# Dependency Tools

Always useful, easy to set up:

- `cargo tree` insight into your dependencies
- `cargo deny` checks licenses and vulnerabilities, useful in your CI
- `cargo audit` checks RUSTSEC database

To gain insight in your supply chain, more advanced:

- `cargo vet`
- `cargo supply-chain`

---
layout: cover
---

# Practical Demo
