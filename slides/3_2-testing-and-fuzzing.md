---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 3.1: Crate Engineering"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 3.1: Crate Engineering"
routerMode: hash
---

# Rust programming

Module 3: Crate Engineering

## Unit 2

Testing your Crate

---
layout: default
---

# Topics of today

- Unit testing

- Fuzzing

- Benchmarking

---
layout: cover
---

# Unit Testing

---
layout: default
---

# Unit testing

You probably already know that this is important!

It is also easy in Rust:

```rust
#[cfg(test)]
mod test {
    user super::*;

    #[test]
    fn my_first_test() {
        assert!(1 != 2);
    }
}
```

<v-click>
`cargo test`:

```
running 1 tests
test test::mt_first_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
</v-click>

---
layout: default
---

# Enhanced Testing

Plenty of crates to help in testing:

- `insta`: test that outputs remain acceptable
   https://docs.rs/insta/latest/insta/

- `ntest`: define test inputs, timeouts
   https://docs.rs/ntest/latest/ntest/

```rust
    fn hello(name: &str) -> String {
        format!("Hello {name}")
    }

    use ntest::test_case;

    #[test_case("hello")]
    #[test_case("Marc")]
    fn test_hello_world(name: &str) {
        insta::assert_debug_snapshot!(hello(name));
    }
```
These are also called `dev-dependencies`; added with `cargo add --dev`

---
layout: cover
---

# Fuzzing

---
layout: default
---

# What is Fuzzing?

- Automated testing technique that provides random data as input
- The program is then monitored for crashes, or assertion failures

- `cargo-fuzz` makes this very easy
  - Requires installing the *nightly* toolchain!

---
layout: default
---

# Example

Meanwhile in `src/lib.rs`:

```rust
pub fn increment_or_die(x: u32) -> u32 {
    if x == 42 { panic!() } else { x+1 }
}
```

<v-click>
In `fuzz_targets/findcrash.rs`:
```rust
use example::increment_or_die;

fuzz_target!(|data: u32| {
    increment_or_die(data);
});
```

<v-click>
Then run:
```bash
cargo +nightly fuzz run findcrash
```
</v-click>
</v-click>

---
layout: default
---

# Output
```
thread '<unnamed>' panicked at /private/bla/src/lib.rs:2:18
```
...
```
Failing input:

	fuzz/artifacts/findcrash/crash-14baecd88cd86197979e9592a3614e57bbd01235

Output of `std::fmt::Debug`:

	42

Reproduce with:

	cargo fuzz run findcrash fuzz/artifacts/findcrash/crash-14baecd88cd86197979e9592a3614e57bbd01235

Minimize test case with:

	cargo fuzz tmin findcrash fuzz/artifacts/findcrash/crash-14baecd88cd86197979e9592a3614e57bbd01235
```

---
layout: default
---

# Writing Fuzz Tests

- Easiest fuzz test: simply detect if crash occurs

- Slightly better: only test "inputs of interest".

- Even better: test if certain properties hold
  - In essence: randomized unit tests!

---
layout: default
---

# Example

In `fuzz_targets/checkoutput.rs`:
```rust
use example::increment_or_die;

fuzz_target!(|data: u32| {
    if data != 42 {
        assert!(increment_or_die(data) > data);
    }
});
```

Q: Will this find any errors?

---
layout: default
---

# Output
```
thread '<unnamed>' panicked at /private/bla/src/lib.rs:2:37
```
...
```
Failing input:

	fuzz/artifacts/checkoutput/crash-14baecd88cd86197979e9592a3614e57bbd01235

Output of `std::fmt::Debug`:

	4294967295

Reproduce with:

	cargo fuzz run checkoutput fuzz/artifacts/checkoutput/crash-14baecd88cd86197979e9592a3614e57bbd01235

Minimize test case with:

	cargo fuzz tmin checkoutput fuzz/artifacts/checkoutput/crash-14baecd88cd86197979e9592a3614e57bbd01235
```

--- 
layout: default
---

# Generating input

- `cargo fuzz` can generate several basic types
- But it does not know about *your* types


<v-click>

- Generating arbitrary input:

```rust
use libfuzzer_sys::arbitrary::{self, Arbitrary};

struct Point2D { x: u8, y: u8 };

impl Arbitrary<'_> for Coord {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Point2D> {
        let x = u.arbitrary()?;
        let y = u.arbitrary()?;
        Ok(Point2D { x, y })
    }
}
```

One can also often use `#[derive(Arbitrary)]`.
</v-click>

---
layout: cover
---
# Further reading:
https://rust-fuzz.github.io/book/

#### Let's find all the the bugs!

---
layout: cover
---
# Benchmarking

---
layout: default
---
# Benchmarking using Criterion

- https://bheisler.github.io/criterion.rs/book/

- Executed using `cargo criterion`
- Does "warm up", outlier detection, detects improvements, ...

```rust {all|5-9|6-8|7|all}
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use example::increment_or_die;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("a very silly benchmark", 
        |b| b.iter(|| 
                increment_or_die(black_box(5))
            )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

- Remember to use `black_box` on inputs!

---
layout: cover
---

# Exercise time!

Try to make our FizzBuzz a little better...

