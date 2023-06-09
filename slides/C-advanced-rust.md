---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - X: Y"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - X: Y'
---
# Rust programming

Module C: Advanced Rust

---
layout: cover
---

# In this module

<!-- Introduce today's subject -->
Dive into advanced Rust topics

---
layout: two-cols
---
# Today

## Part 1: Parallel Rust
- Claim: 'Fearless concurrency;
- Work with threads in Rust

## Part 2: Async foundations
- Introduction Rust `async` programming
- Uncovering `Future` type
- Mechanics behind `async`/`await`
- Running `async` woth Tokio

::right::

## Part 3: FFI
- Working with C from Rust and vice versa
- Work with `cargo bindgen`
- Make nice Rust APIs around C libraries

---
layout: cover
---

# Part 3
Foreign Function Interface

---
layout: default
---
# Q: Why do languages talk with each other?

<v-click>
<div>

- You get an ecosystem for free
- The other language has capabilities (performance, hardware access) that yours doesn't
- You want to gradually introduce a new language in a (large) project
</div>
</v-click>

---
layout: default
---
# Tight langugage coupling

Many languages can use code written in other languages

- JVM: Java, Scala, and Kotlin
- BEAM VM: Erlang and Elixir
- Bare Metal: Zig, D and Nim can import C code

The compiler checks names and types.

---
layout: default
---
# Rust cannot "just" import C code

- Idiomatic C is not idiomatic Rust
- C code cannot provide the guarantees that Rust expects
- Maintaining half of a C compiler is not fun

<v-click>
<div>

Hence, a much looser coupling:

- Generate assembly that is similar to what C generates
- Have the linker stitch everything together
</div>
</v-click>

---
layout: default
---
<img src="https://faultlore.com/blah/c-isnt-a-language/abi-kiss.png" class="ml-50 h-120 rounded shadow" />

---
layout: default
---
# Rust & C disagree

- Different memory layout
- Different calling conventions

---
layout: default
---
# Idea: forward-declare the signature

In rust, this function can now be used like any other

```rust
extern "C" {
    fn my_c_function(x: i32) -> bool;
}
```

The linker will stitch this declaration together with the definition

---
layout: default
---
# How to call a function

```rust
extern "C" {
    fn my_c_function(x: i32) -> bool;
}

pub fn main () {
    unsafe { my_c_function(42) };
}
```

generates this code for `main`:

```asm
example::main:
 push   rax                             # free up rax
 mov    edi,0x2a                        # put the argument into the edi register
 call   80b0 <example::my_c_function>   # call `my_c_function`
 pop    rax                             # restore rax
 ret                                    # return
```

---
layout: default
---
# Space vs Speed

Take this example:
```rust
fn foo(vec: Vec<u8>) -> usize { vec.len() }

fn main() { foo(vec![]); }
```
Recall: Vec = pointer + length + capacity

<v-click>
<div>
We can compile this code in two ways

Using 3 registers:

```rust
fn foo(ptr: *const u8, len: usize, cap: usize) -> usize {
    len
}
```

or using one register and indirection:

```rust
fn foo(vec: *const (usize, usize, usize)) -> usize {
    vec.1
}
```

</div>
</v-click>

---
layout: default
---
# Calling convention

- Rust and C make different choices on by-value vs. by-reference
- `extern "C"` forces Rust to use the C calling convention
- The C calling convention is the 'lingua franca' of calling between languages

---
layout: default
---
# C types != Rust types

For some types, Rust and C agree on the representation

```rust
extern "C" {
    // integers
    fn is_even(x: i32) -> bool;

    // pointers
    fn is_null(ptr: *const u32) -> bool;
}


#[repr(u8)]
enum Color { R, G, B }

extern "C" {
    // tag-only enums
    fn circle_with_me(c: Color) -> Color;
}
```

---
layout: default
---
# C types != Rust types

For others, we must explicitly pick the representation

```rust
#[repr(C)]
struct Point { x: f32, y: f32 }

extern "C" {
    // repr(C) structs
    fn h(p: Point) -> bool;
}

#[repr(transparent)]
struct Wrapper<T>(T);

extern "C" {
    // repr(transparent) structs, if the inner type is repr(C)
    fn h(w: Wrapper<u64>) -> bool;
}
```

---
layout: default
---
# C types != Rust types

For others, we must explicitly pick the representation

```rust
#[repr(C)]
union U { int: i64, float: f64 }

extern "C" {
    // repr(C) unions
    fn i(u: U) -> bool;
}
```

---
layout: default
---
# C types != Rust types

- Many types just don't work:
- Enums like `Result` or `Option`
- Owned collections like `String` and `Vec<T>`
- Fat pointers like `&str` or `&[T]`

These need special, manual treatment

---
layout: default
---
# `cargo-bindgen`

Generates rust API bindings based on C header files

```rust
extern "C" {
    pub fn crypto_stream_salsa20_tweet_xor(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_ulonglong,
        arg4: *const ::std::os::raw::c_uchar,
        arg5: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_16_tweet(
        arg1: *const ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_32_tweet(
        arg1: *const ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
```

---
layout: default
---
# So far

C and Rust don't just work together, we must

- tell rust the name and type of extern functions
- force rust to use the C calling convention
- use only types that have a C-compatible representation
- `cargo-bindgen` automates parts of this process

---
layout: default
---
# Using Rust from C

exposed functions look like this

```rust
#[no_mangle]
extern "C" fn sum(ptr: *const u64, len: usize) -> u64 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    slice.iter().sum()
}
```

Compiling rust into a static library requires some extra setup in the `Cargo.toml`.

---
layout: default
---

# Practice time!
&nbsp;

C3 exercise description: [workshop.tweede.golf](https://workshop.tweede.golf/C3-ffi/mod.html)

---
layout: section
---

# That's a wrap!

Any further questions?