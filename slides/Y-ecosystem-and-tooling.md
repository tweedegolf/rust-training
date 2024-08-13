---
theme: tweedegolf
lineNumbers: true
drawings:
  persist: false
layout: cover
routerMode: hash
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

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    let data = r#" { "name": "John Doe", "age": 43, "phones": [ "+44 1234567", "+44 2345678" ] }"#;

    let p: Person = serde_json::from_str(data)?;

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
```

---
layout: default
---
# `tokio`

de-facto standard for running async programs on non-embedded hardware

---
layout: default
---
# `axum`

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---
layout: default
---
# `clap`

Command Line Interface (CLI) generation

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```

---
layout: default
---
# `clap`

```
$ demo --help
Simple program to greet a person

Usage: demo[EXE] [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version

$ demo --name Me
Hello Me!
```

---
layout: default
---
# `rayon`

Easy data-parallel computation

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

---
layout: default
---
# `tracing`

```rust
pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    // this creates an event at the DEBUG level with two fields:
    // - `excitement`, with the key "excitement" and the value "yay!"
    // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
    //
    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
    if yak == 3 {
        warn!("could not locate yak!");
        // note that this is intended to demonstrate `tracing`'s features, not idiomatic
        // error handling! in a library or application, you should consider returning
        // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!("yak shaved successfully");
    }
    Ok(())
}
```

---
layout: cover
---
# Cargo Commands

---
layout: default
---
# staples

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
# `cargo add`

```sh
$ cargo add regex
$ cargo add serde serde_json -F serde/derive
```

---
layout: default
---
# `cargo doc`

The whole rust ecosystem uses the same docs format

```rust
    /// Creates a constant zero value of this `IntType`.
    ///
    /// # Example
    ///
    /// ```
    /// use inkwell::context::Context;
    /// use inkwell::values::AnyValue;
    ///
    /// let context = Context::create();
    /// let i8_type = context.i8_type();
    /// let i8_zero = i8_type.const_zero();
    ///
    /// assert_eq!(i8_zero.print_to_string().to_string(), "i8 0");
    /// ```
    pub fn const_zero(self) -> IntValue<'ctx> { /* ... */ }
```

documentation can have code snippets

- these are checked automatically (for imports, types, etc)
- asserts are evaluated with `cargo test`

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
# `quickcheck`

A simpler way of quick fuzzing


```rust
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

#[cfg(test)]
mod tests {
  quickcheck! {
      fn prop(xs: Vec<u32>) -> bool {
          xs == reverse(&reverse(&xs))
      }
  }
}
```

---
layout: default
---
# `cargo insta`

Snapshot testing

```rust
#[test]
fn compress_fireworks() {
    let fireworks = include_bytes!("../fireworks.jpg");
    let config = CompressConfig::default();
    insta::assert_debug_snapshot!(compress(config, fireworks));
}
```

tests that this returns the same thing as before

---
layout: default
---
# `cargo nextest`

An alternative test runner

runs every test in a separate process; great for debugging tests with segfaults

```
> cargo nextest run compress_
    Finished test [optimized + debuginfo] target(s) in 0.01s
    Starting 6 tests across 4 binaries (137 skipped)
        PASS [   0.003s] libz-rs-sys tests::deflate::test_compress_bound
        PASS [   0.004s] libz-rs-sys tests::deflate::test_compress_param
        PASS [   0.009s] zlib-rs deflate::test::insufficient_compress_space
        PASS [   0.030s] zlib-rs deflate::test::compress_lcet10
        PASS [   0.065s] zlib-rs deflate::test::compress_paper_100k
        PASS [   0.077s] zlib-rs deflate::test::compress_fireworks
------------
     Summary [   0.078s] 6 tests run: 6 passed, 137 skipped
```

---
layout: default
---
# `cargo expand`

expands macros (both derive and macro_rules!)

```rust
#[derive(Debug)]
struct Foo {
    bar: u128,
    baz: String,
}

// ---

#[automatically_derived]
impl ::core::fmt::Debug for Foo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Foo",
            "bar",
            &self.bar,
            "baz",
            &&self.baz,
        )
    }
}
```

---
layout: default
---
# `cargo bloat` / `cargo llvm-lines`

information about code size (embedded, WASM)

```
> cargo bloat | grep "compiler_builtins"
File  .text   Size             Crate Name
0.1%   5.9%   378B compiler_builtins compiler_builtins::int::specialized_div_rem::u64_div_rem
0.0%   2.1%   138B compiler_builtins compiler_builtins::mem::memcpy
0.0%   1.5%    98B compiler_builtins compiler_builtins::mem::memset
0.0%   0.6%    40B compiler_builtins <u64 as compiler_builtins::int::shift::Ashl>::ashl
0.0%   0.4%    26B compiler_builtins compiler_builtins::arm::__aeabi_memset4
```


