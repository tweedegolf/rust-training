---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 2.4: Traits and Generics"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 2.4: Traits and Generics"
routerMode: hash
---

# Rust programming

Module 2: Foundations of Rust

## Unit 4

Traits and Generics

---
layout: section
---

# Traits and generics

---
layout: default
---

# Generic code for generic types

Functions and methods can make use of generics

```rust
fn reverse_slice<T>(slice: &mut [T]) {
    let n = slice.len();
    for i in 0..n / 2 {
        slice.swap(i, n - i - 1);
    }
}
```

This code gets instantiated for each concrete type

```rust
reverse_slice(&mut [1,2,3]);
reverse_slice(&mut ["foo", "bar", "baz"]);
```

---
layout: default
---

# checking membership

```rust
fn slice_contains<T>(haystack: &[T], needle: &T) -> bool {
    for e in haystack {
        if e == needle {
            return true;
        }
    }

    false
}
```

does this work?

---
layout: default
---

# Generic functions must be valid on their own

independent of for which concrete types they are used in practice

```
error[E0369]: binary operation `==` cannot be applied to type `&T`
 --> src/lib.rs:3:14
  |
3 |         if e == needle {
  |            - ^^ ------ &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn slice_contains<T: std::cmp::PartialEq>(haystack: &[T], needle: &T) -> bool {
  |                    +++++++++++++++++++++
```

---
layout: default
---

# restrict a generic by a trait

```rust
fn slice_contains<T: PartialEq>(haystack: &[T], needle: &T) -> bool {
    for e in haystack {
        if e == needle {
            return true;
        }
    }

    false
}

// or

fn slice_contains<T>(haystack: &[T], needle: &T) -> bool
where
    T: PartialEq
{
    // ...
}
```

---
layout: default
---

# The `PartialEq` trait

```rust
// simplified
pub trait PartialEq {
    // Required method
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    fn ne(&self, other: &Self) -> bool { ... }
}

// Eq is a supertrait of PartialEq
pub trait Eq: PartialEq { }
```

Why `PartialEq`: floats! `NaN` behavior break the laws of `Eq`

---
layout: default
---

# Custom trait `impl`s

```rust
enum BookFormat { Paperback, Hardback, Ebook }
struct Book {
    isbn: i32,
    format: BookFormat,
}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}
impl Eq for Book {}
```

---
layout: default
---

# `#[derive]` a `trait`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BookFormat { Paperback, Hardback, Ebook }

fn main() {
  assert_ne!(BookFormat::Paperback, BookFormat::Ebook);
}
```

- Some traits are trivial to implement
- Derive to quickly implement a trait

---
layout: default
---

# The `PartialEq` trait (for real this time)

```rust
pub trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    // Required method
    fn eq(&self, other: &Rhs) -> bool;

    // Provided method
    fn ne(&self, other: &Rhs) -> bool { ... }
}
```

This allows comparisons between different types, e.g.

```rust
impl<'a> PartialEq<String> for &'a str {
    fn eq(&self, other: String) -> bool {
        *self == other.as_str()
    }
}
```

---
layout: default
---

# Associated types

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 75 provided methods
}
```

https://doc.rust-lang.org/std/iter/trait.Iterator.html

---
layout: default
---

Iterator make `for`-loops work


```rust
for i in [1,2,3] {
    body(i);
}

// is desugared into

let mut it = [1,2,3].iter();
loop {
    match it.next() {
        None => break,
        Some(i) => body(i),
    }
}
```

---
layout: default
---

# Custom iterators

```rust
struct Range {
    start: usize,
    end: usize,
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let current = self.start;
            self.start += 1;
            Some(current);
        } else {
            None
        }
    }
}
```

---
layout: default
---

# How are these different?

```rust
// std version
trait Iterator {
    type Item;
}

// but why not this?
trait Iterator<Item> {
    ...
}

```

---
layout: default
---
# Orphan rule

*Coherence: There must be **at most one** implementation of a trait for any given type*

Trait can be implemented for a type **iff**:
- Either your crate defines the trait
- Or your crate defines the type

Or both, of course

---
layout: section
---

# Common traits from `std`


---
layout: default
---

# Duplication: `std::clone::Clone` & `std::marker::Copy`
```rust{all|9|4-6}
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
      *self = source.clone()
    }
}

pub trait Copy: Clone { } // That's it!
```

- Both `Copy` and `Clone` can be `#[derive]`d
- `Copy` is a marker trait
- `trait A: B` == "Implementor of `A` must also implement `B`"
- `clone_from` has default implementation, can be overridden

---
layout: default
---
# Default values: `std::default::Default`

```rust{all|5|10-17}
pub trait Default: Sized {
    fn default() -> Self;
}

#[derive(Default)] // Derive the trait
struct MyCounter {
  count: u32,
}

// Or, implement it
impl Default for MyCounter {
  fn default() -> Self {
    MyCounter {
      count: 1, // If you feel so inclined
    }
  }
}
```

---
layout: default
---
# Operator overloading: `std::ops::Add<T>` et al.

- Shared behavior

```rust{all|13-14}
use std::ops::Add;
pub struct BigNumber(u64);

impl Add for BigNumber {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
      BigNumber(self.0 + rhs.0)
  }
}

fn main() {
  // Now we can use `+` to add `BigNumber`s!
  let res: BigNumber = BigNumber(1) + (BigNumber(2));
}
```

- Others: `Mul`, `Div`, `Sub`, ..

---
layout: default
---

# Conversion: `Into<T>` & `From<T>`
```rust{all|1-3|5-7|9-15}
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}

impl <T, U> Into<U> for T
  where U: From<T>
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

- Blanket implementation

*Prefer `From` over `Into` if orphan rule allows to*

---
layout: default
---
# Reference conversion: `AsRef<T>` & `AsMut<T>`

```rust
pub trait AsRef<T: ?Sized>
{
    fn as_ref(&self) -> &T;
}

pub trait AsMut<T: ?Sized>
{
    fn as_mut(&mut self) -> &mut T;
}
```

- Provide flexibility to API users
- `T` need not be `Sized`, e.g. slices `[T]` can implement `AsRef<T>`, `AsMut<T>`

---
layout: default
---
# Reference conversion: `AsRef<T>` & `AsMut<T>` (2)

```rust{all|1-2|10-11|13-14}
fn print_bytes<T: AsRef<[u8]>>(slice: T) {
  let bytes: &[u8] = slice.as_ref();
  for byte in bytes {
    print!("{:02X}", byte);
  }
  println!();
}

fn main() {
  let owned_bytes: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
  print_bytes(owned_bytes);

  let byte_slice: [u8; 4] = [0xFE, 0xED, 0xC0, 0xDE];
  print_bytes(byte_slice);
}
```

*Have user of `print_bytes` choose between stack local `[u8; N]` and heap-allocated `Vec<u8>`*

---
layout: default
---
# Markers: `std::marker::Sized`

- Marker traits

```rust
/// Types with a constant size known at compile time.
/// [...]
pub trait Sized { }
```

*`u32` is `Sized`*

*Slice `[T]`, `str` is **not** `Sized`*

*Slice reference `&[T]`, `&str` is `Sized`*

Others:
- `Sync`: Types of which references can be shared between threads
- `Send`: Types that can be transferred across thread boundaries



---
layout: default
---
# Destruction: `std::ops::Drop`

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

- Called when owner goes out of scope


---
layout: three-slots
---
# Destruction:`std::ops::Drop`

::left::
```rust{all|1-7|9-17|19-22}
struct Inner;

impl Drop for Inner {
  fn drop(&mut self) {
    println!("Dropped inner");
  }
}

struct Outer {
  inner: Inner,
}

impl Drop for Outer {
  fn drop(&mut self) {
    println!("Dropped outer");
  }
}

fn main() {
  // Explicit drop
  std::mem::drop(Outer { inner: Inner });
}
```
::right::

<v-click>

<div class="no-line-numbers">
<br/>
Output:
```text
Dropped outer
Dropped inner
```
</div>

- Destructor runs *before* members are removed from stack
- Signature `&mut` prevents explicitly dropping `self` or its fields in destructor
- Compiler inserts `std::mem::drop` call at end of scope

```rust
// Implementation of `std::mem::drop`
fn drop<T>(_x: T) {}
```

*Question: why does `std::mem::drop` work?*

</v-click>

---
layout: cover
---

# Lifetime annotations

---
layout: default
---

# What lifetime?

- References refer to variable
- Variable has a lifetime:
  - Start at declaration
  - End at drop


*Question: Will this compile?*
```rust
/// Return reference to longest of `&str`s
fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

---
layout: default
---
```rust{all|2}
/// Return reference to longest of `&str`s
fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

```
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:2:32
  |
2 | fn longer(a: &str, b: &str) -> &str {
  |              ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
2 | fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
  |          ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
```

---
layout: default
---

# Lifetime annotations

```rust{all|1}
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}
```

English:

- Given a lifetime called `'a`,
- `longer` takes two references `left` and `right`
- that live for <ins>at least</ins> `'a`
- and returns a reference that lives for `'a`

*Note: Annotations do NOT change the lifetime of variables! Their scopes do!*

They just provide information for the borrow checker

---
layout: default
---

# Notation variations

- the `'_` lifetime can be used to let the compiler infer the lifetime
- two `'_` lifetimes in the same type represent **different** lifetimes

```rust
fn foo(a: &'_ str, b: &'_ str) {
    // a and b have different lifetimes now!
}
```

- the static lifetime lives forever
- string literals and constants have a static lifetime

```rust
let s: &'static str = "hello world";
```

---
layout: default
---

# Validating boundaries

- Lifetime validation is done within function boundaries
- No information of calling context is used

*Question: Why?*


---
layout: default
---

# Lifetime annotations in types

```rust
/// A struct that contains a reference to a T
pub struct ContainsRef<'r, T> {
  reference: &'r T
}
```

---
layout: default
---

# Lifetime elision
&nbsp;

Q: "Why haven't I come across this before?"<br/>
<v-click>
<div>
A: "Because of lifetime elision!"
</div>
</v-click>
<v-click>
<div>
<br/>
<br/>

## Rust compiler has heuristics for eliding lifetime bounds:
- Each elided lifetime in input position becomes a distinct lifetime parameter.
- If there is exactly one input lifetime position (elided or annotated), that lifetime is assigned to all elided output lifetimes.
- If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.
- Otherwise, annotations are needed to satisfy compiler
</div>
</v-click>
---
layout: default
---
# Lifetime elision examples

```rust{all|1-2|4-5|7-8|10|12|14-15}
fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL (why?)

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL (why?)

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded
```


---
layout: default
---

# Practice time!

&nbsp;

Unit 2.4 exercise description: [training.tweede.golf](https://training.tweede.golf/traits-and-generics.html)

*Don't forget to* `git pull`!
