# Unit 3.3 - The Tooling of Cargo

<a href="/slides/3_3-cargo/" target="_blank">Slides</a>

## Exercise 3.3.1: DNS Decoding
In this exercise you will practice with unit tests, code coverage, and writing a tiny fuzzer using `cargo-fuzz`.

You will work with a routine `dns_decode` that implements a part of RFC1035 DNS name decoding: in this format, all parts of a domain name are encoded by prefixing them with one byte
indicating the length, followed by the bytes that comprise the part. The domain name itself is zero-terminated (i.e., an "empty part" signifies the end of a domain name).
The periods between parts are not encoded.

So, for example, "mailcrab.tweedegolf.nl" can be encoded as `b"\x08mailcrab\x0Atweedegolf\x02nl\0"`. This is subject to two restrictions:
- The maximum size of a part is 63 bytes.
- The maximum size of a full domain name (including periods) is 255 bytes.

We have presupplied a "first attempt" at `exercises/3-crate-engineering/3-cargo-tooling/1-dns-decode`. You will find the function definition in `src/lib.rs` and
a `src/main.rs` so you can even try the function out interactively. The file `src/lib.rs` even contains some unit tests, so we are off to a great start! Although this function is
not terribly well-tested and contains some bugs. Maybe you can find them?

## Exercise 3.3.1.A Improving Code Coverage
If you haven't done so already, install `cargo-llvm-cov`, by running:
```bash
cargo install cargo-llvm-cov
```
And then the coverage can be viewed by running `cargo llvm-cov` from the crate directory. You can also run `cargo llvm-cov --open` to inspect the coverage report in your browser.

Even though the line coverage is high, as you can see it is not 100%. Write a unit test so that the coverage for `src/lib.rs'` *does* reach 100% line coverage (you will obviously rarely hit this in practice).

## Exercise 3.3.1.B Fuzz Testing
Even with 100% coverage, there are still bugs lurking in here. Let's find them with `cargo-fuzz`. To install that, we not only need to install `cargo-fuzz`, but also use the "nightly" toolchain of Rust, which contains all the experimental features. To install both, run:

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

Then, from `exercises/3-crate-engineering/3-cargo-tooling/1-dns-decode`, run `cargo fuzz init`. This will create a subdirectory `fuzz/` which contains the template for a fuzzing target.
(Note that this `fuzz/` subdirectory is actually a small Rust project, with its own `Cargo.toml` file! So you can add dependencies to it as well, as with any other Rust project!)

You can list all the available targets using:
```bash
cargo +nightly fuzz list
```

You can already run the target `fuzz_target_1` by running
```bash
cargo +nightly fuzz run fuzz_target_1
```
But that will not do find any bugs; edit `fuzz/fuzz_targets/fuzz_target_1.rs` so it does something interesting! Also consider adding more targets using `cargo +nightly fuzz add <TARGETNAME`.

Hints: 
* To access the `dns_decode` function, you need to import it in the fuzzing target using:
  ```rust
  use dns_parse::decode_dns_name;
  ```
* Start simple by writing a fuzzer that just runs `dns_decode` on a `&[u8]` input and see if it detects crashes.

* Try checking the two other properties that must hold for `dns_decode`, that is: 
  - the maximum length of a part is 63 bytes
  - the maximum length of a domain anme is 255 bytes.

* As in unit tests, `assert!` is your friend.

## Exercise 3.3.1: FizzBuzz
In this exercise, you will practise writing a unit test, and use Rusts benchmarking functionality to help you optimize a [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) app. You will need [`cargo-criterion`](https://bheisler.github.io/criterion.rs/book/cargo_criterion/cargo_criterion.html), a tool that runs benchmarks and creates nice reports. You can install it by running

```bash
cargo install cargo-criterion --version=1.1.0
```

### 3.3.1.A Testing Fizz Buzz ⭐
Open `exercises/3-crate-engineering/3-cargo-tooling/1-fizzbuzz/src/lib.rs`. Create a unit test that verifies the correctness of the `fizz_buzz` function. You can use the [`include_str`](https://doc.rust-lang.org/std/macro.include_str.html) macro to include `exercises/3-crate-engineering/3-cargo-tooling/1-fizzbuzz/fizzbuzz.out` as a `&str` into your binary. Each line of `fizzbuzz.out` contains the expected output of the `fizz_buzz` function given the line number as input. You can run the test with

```bash
cargo test
```

By default, Rusts test harness captures all output and discards it, If you like to debug your test code using print statements, you can run

```bash
cargo test -- --nocapture
```

to prevent the harness from capturing output.


### 3.3.1.B Benchmarking Fizz Buzz ⭐⭐
You'll probably have noticed the `fizz_buzz` implementation is not very optimized. We will use `criterion` to help us benchmark `fizz_buzz`. To run a benchmark, run the following command when in the `exercises/3-crate-engineering/3-cargo-tooling/1-fizzbuzz/` directory:

```bash
cargo criterion
```

This command will run the benchmarks, and report some statistics to your terminal. It also generates HTML reports including graphs that you can find under `target/criterion/reports`. For instance, `target/criterion/reports/index.html` is a summary of all benchmark. Open it with your browser and have a look.

Your job is to do some optimization of the `fizz_buzz` function, and use `cargo-criterion` to measure the impact of your changes. Don't be afraid to change the signature of `fizz_buzz`, if, for instance, you want to minimize the number of allocations done by this function. However, make sure that the function is able to correctly produce the output. How fast can you FizzBuzz?

