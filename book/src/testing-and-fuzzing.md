# Unit 3.2 - Testing and Fuzzing

<a href="/slides/3_2-testing-and-fuzzing/" target="_blank">Slides</a>

## Exercise 3.2.1: DNS Decoding
In this exercise you will practice with unit tests, code coverage, and writing a tiny fuzzer using `cargo-fuzz`.

You will work with a routine `dns_decode` that implements a part of RFC1035 DNS name decoding: in this format, all parts of a domain name are encoded by prefixing them with one byte
indicating the length, followed by the bytes that comprise the part. The domain name itself is zero-terminated (i.e., an "empty part" signifies the end of a domain name).
The periods between parts are not encoded.

So, for example, "mailcrab.tweedegolf.nl" can be encoded as `b"\x08mailcrab\x0Atweedegolf\x02nl\0"`. This is subject to two restrictions:
- The maximum size of a part is 63 bytes.
- The maximum size of a full domain name (including periods) is 255 bytes.

We have presupplied a "first attempt" at `exercises/3-crate-engineering/2-testing/1-dns-decode`. You will find the function definition in `src/lib.rs` and
a `src/main.rs` so you can even try the function out interactively. The file `src/lib.rs` even contains some unit tests, so we are off to a great start! Although this function is
not terribly well-tested and contains some bugs. Maybe you can find them?

## Exercise 3.2.1.A Improving Code Coverage
If you haven't done so already, install `cargo-llvm-cov`, by running:
```bash
cargo install cargo-llvm-cov
```
And then the coverage can be viewed by running `cargo llvm-cov` from the crate directory. You can also run `cargo llvm-cov --open` to inspect the coverage report in your browser.

Even though the line coverage is high, as you can see it is not 100%. Write a unit test so that the coverage for `src/lib.rs'` *does* reach 100% line coverage (you will obviously rarely hit this in practice).

## Exercise 3.2.1.B Fuzz Testing
Even with 100% coverage, there are still bugs lurking in here. Let's find them with `cargo-fuzz`. To install that, we not only need to install `cargo-fuzz`, but also use the "nightly" toolchain of Rust, which contains all the experimental features. To install both, run:

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

Then, from `exercises/3-crate-engineering/2-testing/1-dns-decode`, run `cargo fuzz init`. This will create a subdirectory `fuzz/` which contains the template for a fuzzing target.
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
  - the maximum length of a domain name is 255 bytes.

* As in unit tests, `assert!` is your friend.
