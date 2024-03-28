---
theme: tweedegolf
lineNumbers: true
drawings:
  persist: false
layout: cover
---
# Rust programming

Module Y: Ecosystem & Tooling 

<img src="https://raw.githubusercontent.com/tweedegolf/slidev-theme-tweedegolf/1bc81d09e326fcecb531108a5a3bcd9e1856dd84/images/shield-large.png" class=bg-image>

---
layout: cover
---
# Blessed crates 

---
layout: default
---
# `serde` 

serialization & deserialization

has many adapters (json, yaml, binary formats, ect)

---
layout: default
---
# `tokio` 

de-facto standard for running async programs on non-embedded hardware

---
layout: default
---
# `clap` 

Command Line Interface (CLI) generation

Generates the parsing, a help message and good error messages

---
layout: default
---
# `libc` 

For lowlevel things (on unix systems)

---
layout: cover
---
# Cargo Commands 

---
layout: default
---
# stables 

- `cargo fmt`
- `cargo build`
- `cargo run`
- `cargo check`
- `cargo clippy`
- `cargo test`

---
layout: default
---
# `cargo clean` 

Clears the build cache for this project. 

`cargo clean -p foo` clears the artifacts for the `foo` crate

the rust `target` dir grows very large over time

---
layout: default
---
# `cargo +nightly fuzz` 

fuzz testing

```rust
#![no_main]

use libfuzzer_sys::fuzz_target;
use ntp_proto::KeySetProvider;

fuzz_target!(|data: Vec<u8>| {
    let provider = KeySetProvider::dangerous_new_deterministic(1);

    let keyset = provider.get();

    let _ = keyset.decode_cookie_pub(&data);
});
```

---
layout: default
---
# `cargo nextest` 

An alternative test runner 

runs every test in a separate process; great for debugging tests with segfaults

---
layout: default
---
# `cargo expand` 

expands macros (both derive and macro_rules!) 

---
layout: default
---
# `cargo bloat` / `cargo llvm-lines`

information about code size (embedded, WASM)
