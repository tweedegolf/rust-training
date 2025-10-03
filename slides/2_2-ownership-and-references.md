---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 2.2: Ownership and References"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 2.2: Ownership and References"
routerMode: hash
---

# Rust programming

Module 2: Foundations of Rust

## Unit 2

Ownership and References

---

# Learning objectives

- Understand Move Semantics
- Reason about Rust's ownership and borrowing model


---
layout: section
---

# Variable scoping (recap)

```rust
fn main() { // nothing in scope here
    let i = 10; // i is now in scope
    if i > 5 {
        let j = i; // j is now also in scope
        println!("i = {}, j = {}", i, j);
    } // j is no longer in scope, i still remains
    println!("i = {}", i);
} // i is no longer in scope
```

<v-click>

* `i` and `j` are examples containing a `Copy` type
* What if copying is too expensive?

</v-click>

<!--
* When looking at how Rust solves working with the heap, we have to know a little
bit about variable scoping.
* In Rust, every variable has a scope, that is, a section of the code that that
variable is valid for. Note that this isn't that much different to other
programming languages.
* In our example we have `i` and `j`. Note how we can just create a copy by
assigning `i` to `j`.
* Here the type of i and j is actually known as a `Copy` type
* But sometimes there is data that would be way too much to Copy around every
time, it would make our program slow.
-->

---
layout: four-square
---

# Ownership

::topleft::

```rust
let x = 5;
let y = x;
println!("{}", x);
```

::topright::

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
Finished dev [unoptimized + debuginfo] target(s) in 4.00s
Running `target/debug/playground`
5
```

</div>

::bottomleft::

```rust
// Create an owned, heap allocated string
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

Copying a large string can be expensive, we don't want to do that silently.

::bottomright::

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
--> src/main.rs:4:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

</div>

<!--
* Let's take the previous example and get rid of some scopes, instead we are
just going to assign x to y, and then print both x and y. What do we think
is going to happen?
* Now the same example again, but now with a String, "hello", we are just going
to assign it to another variable and then print both s1 and s2. What do we
think is going to happen now?
* See how this time the compiler doesn't even let us run the program. Hold on,
what's going on here?
* Actually, in Rust strings can grow, that means that we can no longer store
them on the stack, and we can no longer just copy them around by re-assigning
them somewhere else.
-->

---

<LightOrDark>
  <template #dark>
    <img src="/images/A1-i-own-this-dark.png" class="pl-30 h-90 float-right" />
  </template>
  <template #light>
    <img src="/images/A1-i-own-this-light.png" class="pl-30 h-90 float-right" />
  </template>
</LightOrDark>

# Ownership

- There is always ever only one owner of a value
- Once the owner goes out of scope the value is cleaned up
- Rust *moves* ownership for non-`Copy` types

<!--
* What we've just seen is the Rust ownership system in action.
* In Rust, every part of memory in use always has an owner variable. That
variable must always be the only owner, there can't be multiple owners.
* Once a scope that contains a variable ends we don't just pop the top from the
stack, but we also clean up any associated values on the heap.
* We can safely do this because we just said that this variable was the only
owner of that part of memory.
* Assigning a variable to another one actually moves ownership to the other
variable and removes it from the first variable, instead of aliasing it
(which is what C and C++ do)
-->

---

# Example

```rust
fn main() {
    let s1 = String::from("Dave");
    display_length(s1);
    println!("Hello {s1}!");
}

fn display_length(name: String) {
    println!("Length = {}", name.len());
}
```

<v-click>

<div class="no-line-numbers">
```text
Compiling playground v0.0.1 (/playground)
 --> src/main.rs:4:29
  |
2 |     let s1 = String::from("Dave");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     display_length(s1);
  |                    -- value moved here
4 |     println!("Hello {s1}!");
  |                           ^^ value borrowed here after move
```
</div>

</v-click>

<!--
* Moving also works when calling a function, the function takes ownership of
the variable that is passed to it
* That means that when the function ends it
will go out of scope and should be cleaned up
* What do you think that will happen in this case when we try and print the
string and the length of the string after the function call.
-->

---

# The compiler has a solution!

<div class="no-line-numbers">
```text
note: consider changing this parameter type in function `say_hello` to borrow instead
 --> src/main.rs:7:20
  |
7 | fn display_length(name: String) {
  |    --------------       ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function

help: consider cloning the value if the performance cost is acceptable
  |
3 |     display_length(s1.clone());
  |                    ++++++++
```
</div>

So, there are two solutions:

- Borrowing
- Cloning

---

# Clone

<img src="/images/A1-clone.jpg" class="float-right w-40" />

- Many (not all!) types in Rust are `Clone`-able
- Cloning must be done explicitly (in contrast, `Copy` is implicit)
- Creating a clone can be expensive and wasteful

```rust
fn main() {
    let s1 = String::from("Dave");
    display_length(s1.clone());
    println!("Hello {s1}!", s1);
}

fn display_length(name: String) {
    println!("Length = {}", name.len());
}
```

<v-click>

Sometimes a `.clone()` is in order, but not in this case!

</v-click>

<!--
* We can solve this problem by "using Clone".
* Many types implement a way to create an explicit copy, such types are
clone-able. But note how we have to very explicitly say that we want a
clone.
* Such a clone is a full deep copy clone and can of course take a long
time, which is why Rust wants you to be explicit.
* Also in this example this is a really inefficient usage of our clone,
because it gets destroyed almost immediately after creation
-->
---
layout: section
---

# Borrowing
- We can make an analogy with real life: if somebody owns something you
  can borrow it from them.
- Borrowing: temporary rights to use the object, but ownership is unchanged.
- Very similar to *call-by-reference* in other languages

```rust 
fn main() {
    let s1 = String::from("Dave");
    display_length(&s1);
    println!("Hello {s1}!");
}

fn display_length(name: &String) {
    println!("Length = {}", name.len());
}
```

---

# References (immutable)

```rust
fn main() {
    let s1 = String::from("Dave");
    change(&s1);
    println!("Hello {s1}!");
}

fn change(name: &String) {
    name.push_str(" Coder");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
8 |     name.push_str(" Coder");
  |     ^^^^ `name` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(name: &mut String) {
  |                  +++
  |
```

</div>

</v-click>

<!--
- Note how we cannot modify the referenced value through an immutable reference
-->

---

# References (mutable)

```rust
fn main() {
    let mut s1 = String::from("Dave");
    change(&mut s1);
    println!("Hello {s1}!");
}

fn change(name: &mut String) {
    name.push_str(" Coder");
}
```

<v-click>

<div class="no-line-numbers">

```text
Hello, Dave Coder
```

</div>

</v-click>

<v-click>

- A mutable reference can even be used to fully replace the original value
- To do this, you can use the dereference operator (`*`) to modify the value:

```rust
fn change(name: &mut String) {
    *name = String::from("Ellen");
}
```

</v-click>

<!--
- We can use a mutable reference here to allow us to modify a borrowed value
- Note that you may also sometimes have to use the deref operator to access
  the value when reading it, but most of the time the Rust compiler will do
  this automatically and you need not worry about it.
-->

---


# Rules for borrowing and references

- You may only ever have **one mutable reference** at the same time
- You may have **any number of immutable references** at the same time **as long as
  there is no mutable reference**
- References cannot *live* longer than their owners
- A reference will always at all times *point to a valid value*

These rules are enforced by Rust's borrow checker.

<!--
- Rust tries to be smart about enforcing these rules, such that you don't notice
  them that often in real life usage, but there may be some cases that clearly
  appear valid, but Rust won't allow. There are generally pretty easy workarounds
  though
- Again: references are not pointers, but in practice of course they do look
  similar and are implemented the same way, but Rust's memory model is not the
  same as that of C/C++ and implementation is not the same as our model.
-->

---

# Borrowing and memory safety
Combined with the ownership model we can be sure that whole classes of errors
cannot occur.

* Rust is memory safe without having to use any runtime background process such
  as a garbage collector
* But we still get the performance of a language that would normally let you
  manage memory manually

<!--
- Memory bugs such as: null pointer dereferences, data races, dangling pointers,
  use after free.
-->

---

# Reference example

```rust
fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s;
    println!("{s1} - {s2} - {s3}");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
3 |     let s1 = &s;
  |              -- immutable borrow occurs here
4 |     let s2 = &s;
5 |     let s3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
6 |     println!("{s1} - {s2} - {s3}");
  |                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `playground` due to previous error
```

</div>

</v-click>

---

# Returning references

You can return references, but the value borrowed from must exist at least as
long

```rust
fn give_me_a_ref() -> &String {
    let s = String::from("Hello, world!");
    &s
}
```

<v-click>

<div class="no-line-numbers">

```md {8}
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:23
  |
1 | fn give_me_a_ref() -> &String {
  |                       ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | fn give_me_a_ref() -> &'static String {
  |                       ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
```

</div>

</v-click>

---

# Returning references

You can return references, but the value borrowed from must exist at least as
long

```rust
fn give_me_a_ref(input: &(String, i32)) -> &String {
    &input.0
}
```

<v-click>

```rust
fn give_me_a_value() -> String {
    let s = String::from("Hello, world!");
    s
}
```

</v-click>


---
layout: default
---

# Practice time!

&nbsp;

Unit 2.2 exercise description: [training.tweede.golf](https://training.tweede.golf/ownership-and-references.html)

*Don't forget to* `git pull`!
