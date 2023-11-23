---
theme: tweedegolf
lineNumbers: true
fonts:
  mono: 'IBM Plex Mono'
  sans: 'IBM Plex Sans'
drawings:
  persist: false
layout: cover
---

# Rust programming

Module A: Introduction to Rust

<img src="https://raw.githubusercontent.com/tweedegolf/slidev-theme-tweedegolf/1bc81d09e326fcecb531108a5a3bcd9e1856dd84/images/shield-large.png" class=bg-image>

---
layout: cover
---

# In this module

<!-- Introduce today's subject -->
An introduction to the Rust language and its features

---
layout: two-cols
---

# Overview
**Part 1: Basic Syntax**
- Get acquainted with Rust
- Work with basic syntax and operators

<br/>
<br/>

**Part 2: Ownership and References**
- Understand the rules of the Rust ownership model

::right::
**Part 3: Advanced Syntax**
- Work with more advanced syntax and operators

<br/>
<br/>
<br/>

**Part 4: Traits & Generics**
- Understand Rust type generics
- Work with traits to make code generic

---
layout: cover
---

# Part 1
Basic Syntax

---
layout: section
---

# Basic Syntax

---
layout: default
---

# A new project

```bash
$ cargo init hello-world
```

<v-click>

```bash
$ cd hello-world
$ cargo run
```

</v-click>

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.74s
Running `target/debug/hello-world`
Hello, world!
```

</div>

</v-click>


---

# Hello, world!

```rust {all|1-3|2|5-11|6-10|7,9|all}
fn main() {
    println!("Hello, world! fib(6) = {}", fib(6));
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.28s
Running `target/debug/hello-world`
Hello, world! fib(6) = 8
```

</div>

</v-click>

<!--
- `fn main()` is the entrypoint of your program
- `println!` (output something to stdout)
- Note the call syntax `fib(6)` with comma separated parameters
- exclamation mark is a macro (we'll see later)
- `fn` short for function, declare a function
- we see our first types here, we'll see more about them later
- `u64` unsigned integer types, all integers have an explicit size, 64 bits in
this case
- `if-else` is without parenthesis for the expression, but with required braces
for the blocks
- no explicit return keyword (will get back to that)
-->

---

# Variables

```rust {all|2|all}
fn main() {
    let some_x = 5;
    println!("some_x = {}", some_x);
    some_x = 6;
    println!("some_x = {}", some_x);
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
error[E0384]: cannot assign twice to immutable variable `some_x`
--> src/main.rs:4:5
  |
2 |     let some_x = 5;
  |         ------
  |         |
  |         first assignment to `some_x`
  |         help: consider making this binding mutable: `mut some_x`
3 |     println!("some_x = {}", some_x);
4 |     some_x = 6;
  |     ^^^^^^^^^^ c annot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `hello-world` due to previous error
```

</div>

</v-click>

<!--
- By convention rust uses snake case (i.e. all lowercase with underscores) for
variable names
- The immutable variable cannot be mutated in any way (exceptions apply)
-->

---

# Variables

```rust
fn main() {
    let mut some_x = 5;
    println!("some_x = {}", some_x);
    some_x = 6;
    println!("some_x = {}", some_x);
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.26s
Running `target/debug/hello-world`
some_x = 5
some_x = 6
```

</div>

</v-click>

<!--
- We declare a mutable variable by adding `mut`, we can update the value for
that variable
-->

---

# Assigning a type to a variable

```rust
fn main() {
    let x: i32 = 20;
}
```

- Rust is strongly and strictly typed
- Variables use type inference, so no need to specify a type
- We can be explicit in our types (and sometimes have to be)

---
layout: two-cols
---

# Integers

| Length        | Signed  | Unsigned |
| ------------- | ------- | -------- |
| 8 bits        | `i8`    | `u8`     |
| 16 bits       | `i16`   | `u16`    |
| 32 bits       | `i32`   | `u32`    |
| 64 bits       | `i64`   | `u64`    |
| 128 bits      | `i128`  | `u128`   |
| pointer-sized | `isize` | `usize`  |

- Rust prefers explicit integer sizes
- Use `isize` and `usize` sparingly

::right::

<v-click>

# Literals

```rust
fn main() {
    let x = 42; // decimal as i32
    let y = 42u64; // decimal as u64
    let z = 42_000; // underscore separator

    let u = 0xff; // hexadecimal
    let v = 0o77; // octal
    let w = 0b0100_1101; // binary
    let q = b'A'; // byte syntax (stored as u8)
}
```

</v-click>

<!--
- Use isize and usize mostly when working with indexing or other things
that need to have a specific size for your platform
-->

---

# Floating points and floating point literals

```rust
fn main() {
    let x = 2.0; // f64
    let y = 1.0f32; // f32
}
```

- `f32`: single precision (32 bit) floating point number
- `f64`: double precision (64 bit) floating point number

<!--
- Rust uses f64 by default
- Similar to integers you can append the type of float to indicate a specific
literal type
-->

---

# Numerical operations

```rust
fn main() {
    let sum = 5 + 10;
    let difference = 10 - 3;
    let mult = 2 * 8;
    let div = 2.4 / 3.5;
    let int_div = 10 / 3; // 3
    let remainder = 20 % 3;
}
```

<v-click>

- These expressions do overflow/underflow checking in debug
- In release builds these expressions are wrapping, for efficiency
- You cannot mix and match types here, not even between different integer
types

```rust
fn main() {
    let invalid_div = 2.4 / 5;          // Error!
    let invalid_add = 20u32 + 40u64;    // Error!
}
```

</v-click>

<!--
- Rust has your typical operations, just as with other C-like languages
-->

---

# Booleans and boolean operations

```rust
fn main() {
    let yes: bool = true;
    let no: bool = false;
    let not = !no;
    let and = yes && no;
    let or = yes || no;
}
```

---

# Comparison operators

```rust
fn main() {
    let x = 10;
    let y = 20;
    x < y; // true
    x > y; // false
    x <= y; // true
    x >= y; // false
    x == y; // false
    x != y; // true
}
```

Note: as with numerical operators, you cannot compare different integer and
float types with each other

```rust
fn main() {
    3.0 < 20; // invalid
    30u64 > 20i32; // invalid
}
```

<!--
- Boolean operators short-circuit: i.e. if in `a && b`, a is already false,
then the code for b is not executed
-->

---

# Characters

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

- A character is a 32 bit unicode scalar value
- Very much unlike C/C++ where char is 8 bits

<!--
- The final scalar type is the character, but it isn't often seen.
- Note that it is not the same as u8 (a byte) in rust, and cannot be used
interchangeably.
- We'll see later that strings do not use chars, but are encoded as UTF-8
instead.
-->

---

# Strings
```rust
    // Owned, heap-allocated string *slice*
    let s1: String = String::from("Hello, 🌍!");
```

- Rust strings are UTF-8-encoded
- Unlike C/C++: *Not null-terminated*
- Cannot be indexed like C Strings
- Actually many types of strings in Rust

<!--
- Rusts strings are complicated, because all strings are complicated
- Rusts strings are UTF-8 encoded sequences. Not null terminated unlike C/C++
- Literal strings are static by default, called string *slices*, being pointers to their start, accompanied with the length
-->

---
layout: three-slots
---
# Tuples

::left::

```rust
fn main() {
    let tup: (i32, f32, char) = (1, 2.0, 'a');
}
```

- Group multiple values into a single compound type
- Fixed size
- Different types per element
- Create a tuple by writing a comma-separated list of values inside parentheses

::right::

<v-click>

<div>
```rust
fn main() {
    let tup = (1, 2.0, 'Z');
    let (a, b, c) = tup;
    println!("({}, {}, {})", a, b, c);

    let another_tuple = (true, 42);
    println!("{}", another_tuple.1);
}
```

- Tuples can be destructured to get to their individual values
- You can also access individual elements using the period operator followed by
  a zero based index
</div>
</v-click>

<!--
- Note how the tuple type and the tuple value are constructed similarly, but
the type contains individual element types
- We will see more powerful variants of this destructuring later
- Note that after destructuring the original value is no longer accessible
-->

---

# Arrays

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr[0]);
    let [a, b, c] = arr;
    println!("[{}, {}, {}]", a, b, c);
}
```

- Also a collection of multiple values, but this time all of the same type
- Always a fixed length at compile time (similar to tuples)
- Use square brackets to access an individual value
- Destructuring as with tuples
- Rust always checks array bounds when accessing a value in an array

<!--
- Create an array by writing a comma-separated list of values inside brackets
- Note how unlike C/C++ arrays must always have a length defined at compile
time and cannot be constructed dynamically
- You can also construct an array using [value; repetitions] instead of having
to write out each value if you have a repeating value.
- For the type declaration the element type and count are separated by a semicolon and
written between brackets
-->

---

# Control flow

```rust {all|3-10|4-9|8|13-16|18-20|all}
fn main() {
    let mut x = 0;
    loop {
        if x < 5 {
            println!("x: {}", x);
            x += 1;
        } else {
            break;
        }
    }

    let mut y = 5;
    while y > 0 {
        y -= 1;
        println!("y: {}", y);
    }

    for i in [1, 2, 3, 4, 5] {
        println!("i: {}", i);
    }
}
```

<!--
- A loop or if condition must always evaluate to a boolean type, so no `if 1`
- Use break to break out of a loop, also works with for and while, continue
to skip to the next iteration
-->

---

# Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn returns_nothing() -> () {
    println!("Nothing to report");
}

fn also_returns_nothing() {
    println!("Nothing to report");
}
```

- The function boundary must always be explicitly annotated with types
- Within the function body type inference may be used
- A function that returns nothing has the return type *unit* (`()`)
- The function body contains a series of statements optionally ending with an
expression

<!--
- Rust always uses snake case for variables and functions
- We must annotate each function parameter with a type, using a colon
- We must annotate the function return type using an arrow (`->`) followed by
the return type
- Unit may be omitted, note the syntax looks like an empty tuple: a tuple with
no value members has no instances, just as with unit.
- In rust you must always specify your type signatures for function boundaries
-->

---

# Statements
- Expressions evaluate to a resulting value
- Statements are instructions that perform some action and do not return a value
- A definition of any kind (function definition etc.)
- The `let var = expr;` statement
- Almost everything else is an expression

## Example statements
```rust
fn my_fun() {
    println!("{}", 5);
}
```

```rust
let x = 10;
```

<v-click>

```rust
let x = (let y = 10); // invalid
```

</v-click>

<!--
- Note how `let` within a `let` is not allowed because of `let` being a statement,
thus you may not declare multiple variables at the same time with the same
value
- `let` is a statement because ownership makes multiple assignments behave
differently than many would expect, it is almost never what you want in
Rust
- It also makes sense if you think of all other declarations also being
statements
-->

---

# Expressions

- Expressions make up most of the Rust code you write
- Includes some of the control flow such as `if` and `loop`
- Includes scoping braces (`{` and `}`)
- An expression can be turned into a statement by adding a semicolon (`;`)

```rust {all|2-5}
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y); // 4
}
```

---

# Expressions - control flow

- Control flow expressions as a statement do not need to end with a semicolon
if they return *unit* (`()`)
- Remember: A block/function can end with an expression, but it needs to have
the correct type

```rust {all|3-8|10-15}
fn main() {
    let y = 11;
    // if as an expression
    let x = if y < 10 {
        42
    } else {
        24
    };

    // if as a statement
    if x == 42 {
        println!("Foo");
    } else {
        println!("Bar");
    }
}
```

---

# Scope

- We just mentioned the scope braces (`{` and `}`)
- Variable scopes are actually very important for how Rust works

```rust
fn main() {
    println!("Hello, {}", name);  // invalid: name is not yet defined
    let name = "world";  // from this point name is in scope
    println!("Hello, {}", name);
} // name goes out of scope
```

---

# Scope

As soon as a scope ends, all variables for that scope can be removed from the
stack

```rust
fn main() { // nothing in scope here
    let i = 10; // i is now in scope
    if i > 5 {
        let j = 20; // j is now also in scope
        println!("i = {}, j = {}", i, j);
    } // j is no longer in scope, i still remains
    println!("i = {}", i);
} // i is no longer in scope
```

<!--
- Note that this is the same with C and C++
-->

---
layout: default
---

# Practice time!

&nbsp;

A1 exercise description: [workshop.tweede.golf](https://workshop.tweede.golf/A1-basic-syntax/mod.html)

*Don't forget to* `git pull`!

---
layout: cover
---

# Part 2
Ownership and References

---
layout: two-cols
---
# Memory management

- Most of what we have seen so far is stack-based and small in size
- All these primitive types are `Copy`: create a copy on the stack every time
we need them somewhere else
- We don't want to pass a copy all the time
- Large data that we do not want to copy
- Modifying original data
- What about datastructures with a variable size?

::right::

<Transform scale="0.85">

![Memory Layout](/images/A1-memory-expanded.svg)

</Transform>


---
layout: default
---
# Memory

- A computer program consists of a set of instructions
- Those instructions manipulate some memory
- How does a program know what memory can be used?

<!--
* A program is not just the code that is running, it is also the current state
of that program (the memory).
* But central here is the question: when does a program know when it can use
a specific part of that memory, when is it available?
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-250px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-5">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-12 left-26">🠔 Stack pointer</div>
    </div>
</div>

<!--
* In this simplified view we see the stack mechanism and the heap mechanism
* The stack is a growing stack of used memory, where the only way to remove
memory from being used is by removing it from the top of the stack and the
only way to add is to put it on top of the stack.
* Somehow, as with a lot of CS stuff, we like to turn things around and think
of stacks growing down instead of up in the real world. That is because they are
at the end of the virtual memory address range. So if the stack grows, the stack
pointer (to the current stack frame) is decreased.
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-250px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 3</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-5">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-19 left-26">🠔 Stack pointer</div>
        <div class="relative pl-27 top-20">
            A stack frame is allocated for every function call. It contains exactly
            enough space for all local variables, arguments and stores where the
            previous stack frame starts.
        </div>
    </div>
</div>

<!--
* We create a new part of the stack, called stack frame, every time we enter a function, meanwhile
we have a small special bit of memory, register, where the current top of the stack is
recorded.
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-250px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-5">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-12 left-26">🠔 Stack pointer</div>
        <div class="relative pl-27 top-13">
            Once a function call ends we just move back up, and everything below is
            available as free memory once more.
        </div>
    </div>
</div>

<!--
* And as we leave a function, we just put the stack pointer back down and we
just act as if everything above it doesn't exist.
* Also take a look at the heap memory instead, look at how there are many
differently sized blocks of memory scattered across the heap.
-->

---

# Stack limitations

The stack has limitations though, because it only grows as a result of a
function call.

* Size of items on stack frame must be known at compile time
* If I don't know the size of a variable upfront: What size should my stack
frame be?
* How can I handle arbitrary user input efficiently?

<style>
    .footnotes-sep {
        margin-top: 45px;
    }

    .footnotes {
        @apply text-xs opacity-65;
    }

    .footnote-backref {
        display: none;
    }
</style>

<!--
* You can definitely do a lot with just a stack, but really there are some
scenarios that aren't possible, or can only be done very inefficient when
we can only ever push and pop from the top of the stack.
* Because stack frames (at least for low level compiled languages such as Rust,
C and C++) need to be known at compile time, we also have somewhat limited
capabilities for dynamic variable sizes and dynamic user input
* Note that stack based operations are very much a solved problem, and you can
very safely use stack based variables in C and C++, because you don't have to
worry about cleaning them up, there are no pointers.
-->

---

# The Heap

If the lifetime of some data needs to outlive a certain scope, it can not be placed on the stack.
We need another construct: the heap.

It's all in the name, the heap is just one big pile of memory for you to store
stuff in. But what part of the heap is in use? What part is available?

* Data comes in all shapes and sizes
* When a new piece of data comes in we need to find a place in the heap that
still has a large enough chunk of data available
* When is a piece of heap memory no longer needed?
* Where does it start? Where does it end?
* When can we start using it?

<!--
* Meanwhile on the other side of our memory the heap is an unstructured pile
of data just waiting to be used. But how do we know what to use, when to use,
when to stop using? We can't keep on adding more and more memory or we would
run into a runaway memory leak quickly.
* Let's take a look how Rust solves working with the heap for us.
-->

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
<div>

* `i` and `j` are examples containing a `Copy` type
* What if copying is too expensive?
</div>
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

# Move semantics

::topleft::

```rust
let x = 5;
let y = x;
println!("{}", x);
```

::topright::

<div class="no-line-numbers">

<v-click>

```text
Compiling playground v0.0.1 (/playground)
Finished dev [unoptimized + debuginfo] target(s) in 4.00s
Running `target/debug/playground`
5
```

</v-click>

</div>

::bottomleft::

<v-click>

```rust
// Create an owned, heap allocated string
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

</v-click>

<v-click at="4">

Strings store their data on the heap because they can grow

</v-click>

::bottomright::

<v-click at="3">

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

</v-click>

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

<img src="/images/A1-i-own-this.png" class="pl-30 h-90 float-right" />

# Ownership

- *There is always ever only one owner (variable) of a stack value*
- Once the owner goes out of scope (and is removed from the stack), any associated values on the
  heap will be cleaned up as well
- Rust transfers ownership for non-copy types: *move semantics* 

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

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
--> src/main.rs:4:43
  |
2 | let s1 = String::from("hello");
  |     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 | let len = calculate_length(s1);
  |                            -- value moved here
4 | println!("The length of '{}' is {}.", s1, len);
  |                                       ^^ value borrowed here after move
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

# Moving out of a function

We can return a value to move it out of the function

```rust
fn main() {
    let s1 = String::from("hello");
    let (len, s1) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
Finished dev [unoptimized + debuginfo] target(s) in 5.42s
Running `target/debug/playground`
The length of 'hello' is 5.
```

</div>

</v-click>

<!--
* But what if we move a value into a function and we still want to use it
afterwards, we could choose to move it back at the end of the function, but
it really doesn't make for very nice code
* Note that Rust allows us to return multiple values from a function with
this syntax.
-->

---

# Clone

<img src="/images/A1-clone.jpg" class="float-right w-40" />

- Many types in Rust are `Clone`-able
- `Clone::clone()` is an *explicit* clone (`Copy` is *implicit*)
- Can be expensive

```rust
fn main() {
    let x = String::from("hellothisisaverylongstring...");
    let len = get_length(x.clone());
    println!("{}: {}", x, len);
}

fn get_length(arg: String) -> usize {
    arg.len()
}
```

<!--
* There is something else in Rust
* Many types implement a way to create an explicit copy, such types are
clone-able. But note how we have to very explicitly say that we want a
clone. 
* Such a clone is a full deep copy clone and can of course take a long
time, which is why Rust wants you to be explicit. 
* Also in this example this is a really inefficient usage of our clone, 
because it gets destroyed almost immediately after creation
-->



---

# Ownership
We previously talked about ownership

* In rust there is always a single owner for each stack value
* Once the owner goes out of scope any associated values should be cleaned up
* Copy types creates copies, all other types are *moved*

<!--
- Note once more that the idea of moving is something that exists in the Rust
  world, but not necesarrily every move actually copies bytes around, these are
  all things where Rust's model is an abstraction over what the compiled code
  actually does.
-->

---

# Moving out of a function
We have previously seen this example


```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: String) -> usize {
    s.len()
}
```

* This does not compile because ownership of `s1` is moved into
 `calculate_length`, meaning it is no longer available in `main` afterwards
* We can use `Clone` to create an explicit copy
* We can give ownership back by returning the value
* What about other options?

---

# Borrowing
- We can make an analogy with real life: if somebody owns something you can
  borrow it from them, but eventually you have to give it back
- If a value is borrowed, it is not moved and the ownership stays with the
  original owner
- To borrow in rust, we create a *reference* (&ne; pointer)

```rust {all|3|7|all}
fn main() {
    let x = String::from("hello");
    let len = get_length(&x);
    println!("{}: {}", x, len);
}

fn get_length(arg: &String) -> usize {
    arg.len()
}
```

---

# References (immutable)

```rust
fn main() {
    let s = String::from("hello");
    change(&s);
    println!("{}", s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `playground` due to previous error
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
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 2.55s
     Running `target/debug/playground`
hello, world
```

</div>

</v-click>

<v-click>
<div>

- A mutable reference can even fully replace the original value
- To do this, you can use the dereference operator (`*`) to modify the value:

```rust
*some_string = String::from("Goodbye");
```
</div>
</v-click>

<!--
- We can use a mutable reference here to allow us to modify a borrowed value
- Note that you may also sometimes have to use the deref operator to access
  the value when reading it, but most of the time the Rust compiler will do
  this automatically and you need not worry about it.
-->

---
layout: two-cols
---

# Rules borrowing and references

<v-click>

1. Only ever *one* mutable *reference* at the same time
</v-click>
<v-click>

2. *Any* number of immutable *references* at the same time *if no mutable reference* exists
</v-click>
<v-click>

4. References *cannot live longer* than their owners

</v-click>
<v-click>

5. Reference *always points to a valid value*
</v-click>
<v-click>


Rust's *Borrow Checker* checks this
</v-click>

::right::
<img src="/images/A1-borrow-checker.png" class="pl-30 float-right" />
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
Whole classes of errors cannot occur:

* Rust is memory safe without having to use any runtime background proces such
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
    println!("{} - {} - {}", s1, s2, s3);
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
6 |     println!("{} - {} - {}", s1, s2, s3);
  |                              -- immutable borrow later used here

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

A2 exercise description: [workshop.tweede.golf](https://workshop.tweede.golf/A2-ownership-references/mod.html)

---
layout: cover
---

# Part 3
Advanced Syntax

---

# Types redux
We have previously looked at some of the basic types in the Rust typesystem

- Primitives (integers, floats, booleans, characters)
- Compounds (tuples, arrays)
- Most of the types we looked at were `Copy`
- Borrowing will make more sense when we look at some more ways we can type
  our data

---

# Structuring data
Rust has two important ways to structure data

* structs
* enums
* ~~unions~~

<!--
- We have unions in Rust, but almost everywhere you will use enums instead.
  Unions become relevant once we start talking about FFI and unsafe Rust code.
-->

---

# Structs
A struct is similar to a tuple, but this time the combined type gets its own name

```rust
struct ControlPoint(f64, f64, bool);
```

<v-click>

This is an example of a *tuple struct*. You can access the fields in the struct
the same way as with tuples:

```rust
fn main() {
  let cp = ControlPoint(10.5, 12.3, true);
  println!("{}", cp.0); // prints 10.5
}
```

</v-click>

<!--
- Note that two tuples with the same fields in the same order are always the
  same type, whereas two structs with different names but the same fields are
  different types.
-->

---

# Structs
Much more common though are structs with named fields

```rust
struct ControlPoint {
  x: f64,
  y: f64,
  enabled: bool,
}
```

* We can add a little more purpose to each field
* No need to keep our indexing up to date when we add or remove a field

<v-click>


```rust {all|2-6|7}
fn main() {
  let cp = ControlPoint {
    x: 10.5,
    y: 12.3,
    enabled: true,
  };
  println!("{}", cp.x); // prints 10.5
}
```

</v-click>

<!--
- Named fields are especially easier in usage, as a type alone will most of
  the time not be enough information to determine the full meaning, here we now
  now that the two floats meant the x and y coordinates and we know what the
  boolean indicated.
- To instantiate (create a value) of a struct we use the syntax shown
- Now, we can use the same `x.y` syntax as with tuples, but we have a nice
  name for referencing our fields instead of having to remember the exact
  field index.
-->

---

# Enumerations
One of the more powerful kinds of types in Rust are enumerations

```rust
enum IpAddressType {
  Ipv4,
  Ipv6,
}
```

* An enumeration (listing) of different *variants*
* Each variant is an alternative value of the enum, you pick a single value to
  create an instance

<v-click>

```rust
fn main() {
  let ip_type = IpAddressType::Ipv4;
}
```

</v-click>

---

# Enumerations
But enums get more powerful, because each variant can have associated data with
it

```rust
enum IpAddress {
  Ipv4(u8, u8, u8, u8),
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),
}
```

* This way, the associated data and the variant are bound together
* Impossible to create an ipv6 address while only giving a 32 bits integer

```rust
fn main() {
  let ipv4_home = IpAddress::Ipv4(127, 0, 0, 1);
  let ipv6_home = IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1);
}
```

* Note: an enum always is as large as the largest variant

<div class="relative" style="background-color: lightgrey; padding-right: 10px">

![Memory Layout](/images/A2-enum-memory.drawio.svg)

</div>

---

# Pattern matching
To extract data from enums we can use pattern matching using the
`if let [pattern] = [value]` statement

```rust
fn accept_ipv4(ip: IpAddress) {
  if let IpAddress::Ipv4(a, b, _, _) = ip {
    println!("Accepted, first octet is {} and second is {}", a, b);
  }
}
```

* `a` and `b` introduce local variables within the body of the if that contain
  the values of those fields
* The underscore (`_`) can be used to accept any value

---

# Match
But pattern matching is very powerful if combined with the match statement

```rust
fn accept_home(ip: IpAddress) {
  match ip {
    IpAddress::Ipv4(127, 0, 0, 1) => {
      println!("You are home!");
    },
    IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1) => {
      println!("You are in your new home!");
    },
    _ => {
      println!("You are not home");
    },
}
```

* Every part of the match is called an arm
* A match is exhaustive, which means that all values must be handled by one of
  the match arms
* You can use a catch-all `_` arm to catch any remaining cases if there are any
  left

---

# Generics
Enums become even more powerful if we introduce a little generics

```rust
struct PointFloat(f64, f64);
struct PointInt(i64, i64);
```

We are repeating ourselves here, what if we could write a datastructure for
both of these cases?

<v-click>
<div>

```rust
struct Point<T>(T, T);

fn main() {
  let float_point: Point<f64> = Point(10.0, 10.0);
  let int_point: Point<i64> = Point(10, 10);
}
```

Generics are much more powerful, but this is all we need for now
</div>
</v-click>

<!--
* The upper case letter between the angled brackets introduces a generic type
  parameter.
* We can now use that generic type variable we introduced as a type name
* Then at the point of using the type we can specify which actual type we
  want to use
* Generics are much more powerful, but this is enough for now
-->

---

# Option
A quick look into the basic enums available in the standard library

* Rust does not have null, but you can still define variables that optionally
  do not have a value
* For this you can use the `Option<T>` enum

```rust
enum Option<T> {
  Some(T),
  None,
}

fn main() {
  let some_int = Option::Some(42);
  let no_string: Option<String> = Option::None;
}
```

<!--
* Note how Rust can infer the type of `some_int`, but we have to specify what
  the type of the Option is in the None case, because it cannot possibly know
  what kind of values you could put in that Option
* Also not that for normal enums we have to import the variants, but Option
  is so common that the variants are available by default without needing to
  prefix them with `Option::`
-->

---

# Option
A quick look into the basic enums available in the standard library

* Rust does not have null, but you can still define variables that optionally
  do not have a value
* For this you can use the `Option<T>` enum

```rust
enum Option<T> {
  Some(T),
  None,
}

fn main() {
  let some_int = Some(42);
  let no_string: Option<String> = None;
}
```

*`Option::Some` and `Option::None` are included with Rust's **prelude***
---

# Error handling
What would we do when there is an error?

```rust
fn divide(x: i64, y: i64) -> i64 {
  if y == 0 {
    // what to do now?
  } else {
    x / y
  }
}
```

---

# Panic! 😱

```rust
fn divide(x: i64, y: i64) -> i64 {
  if y == 0 {
    panic!("Cannot divide by zero");
  } else {
    x / y
  }
}
```

* Most basic way to handle errors
* Will immediately stop the current thread/program and instead shut it down, by either:
  * Stack unwinding
  * Aborting
* Only `panic!` if error is unrecoverable
* Don't use in libraries!

<!--
* Unwinding has its usages, mainly to clean up resources that you previously
  opened.
* An unwind can be stopped, but this is highly unusual to do and very expensive
* In a multithreaded program unwinding is essential to make sure that any
  memory owned by that thread is freed, making sure you don't have any memory
  leaks
* Rust programs are compiled such that if a panic does not occur, it doesn't
  add any extra cost, but that does mean that if a panic does occur, it isn't
  very fast
* Generally panicing should be avoided as much as possible
* The panic! macro is not the only way to trigger a panic, so beware, we will
  see some ways we can also trigger a panic very soon
* Note that if the main thread panics, the entire program will always exit
-->

---

# Result
The non-nuclear option

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}

enum DivideError {
  DivisionByZero,
  CannotDivideOne,
}

fn divide(x: i64, y: i64) -> Result<i64, DivideError> {
  if x == 1 {
    Err(DivideError::CannotDivideOne)
  } else if y == 0 {
    Err(DivideError::DivisionByZero)
  } else {
    Ok(x / y)
  }
}
```

---

# Handling results

```rust
fn div_zero_fails() {
  match divide(10, 0) {
    Ok(div) => println!("{}", div),
    Err(DivideError::DivisionByZero) => println!("Could not divide by zero"),
    Err(DivideError::CannotDivideOne) => println!("Apparently cannot devide 1"),
  }
}
```

* `divide` function signature is explicit in how it can fail
* Let user decide how to handle errors
* `Result::Ok` and `Result::Err` are in Rust prelude


<!--
- Note how in this case the error still causes a panic, but at least we get a
  choice of what we do
-->

---

# `unwrap()`-ing `Results`

```rust
fn div_zero_fails() {
  let div = divide(10, 0).unwrap();
  println!("{}", div);
}
```

* Unwraps return inner value in case of `Result::Ok` or `Option::Some`
* Panicks otherwise with error message
* Use only if you're sure panic won't happen!

---

# Result and the `?` operator
Results are so common that there is a special operator associated with them, the
`?` operator

```rust
fn can_fail(n: i32, m: i32) -> Result<i64, Error> {
  let intermediate_result = match divide(10, n) {
    Ok(ir) => ir,
    Err(e) => return Err(e);
  };

  match divide(intermediate_result, m) {
    Ok(sec) => Ok(sec * 2),
    Err(e) => Err(e),
  }
}
```

<v-click>

Look how this function changes if we use the `?` operator

```rust
fn can_fail(n: i32, m: i32) -> Result<i64, Error> {
  let intermediate_result = divide(10, n)?;
  Ok(divide(intermediate_result, m)? * 2)
}
```

</v-click>

---

# Intermission: Impl blocks
In the past few slides we saw a syntax which wasn't explained before:

```rust {3}
fn main() {
  let x = Some(42);
  let unwrapped = x.unwrap();
  println!("{}", unwrapped);
}
```

* The syntax `x.y()` looks similar to how we accessed a field in a struct
* We can define functions on our types using impl blocks
* Impl blocks can be defined on any type, not just structs (with some limitations)

---

# Intermission: Impl blocks

```rust {all|6,13|7-12|7|17}
enum IpAddress {
  Ipv4(u8, u8, u8, u8),
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),
}

impl IpAddress {
  fn as_u32(&self) -> Option<u32> {
    match self {
      IpAddress::Ipv4(a, b, c, d) => a << 24 + b << 16 + c << 8 + d
      _ => None,
    }
  }
}

fn main() {
  let addr = IpAddress::Ipv4(127, 0, 0, 1);
  println!("{:?}", addr.as_u32());
}
```

<!--
- Here we define the as_u32 method
- Note how the impl block is separate from the type definition
- In fact we can have multiple impl blocks for the same type, as long as
  function definitions do not overlap (not useful right now, but it will be
  once we get more into generics)
-->

---

# Intermission: `impl` blocks, `self` and `Self`

- The `self` parameter defines how the method can be used.
- The `Self` type is a shorthand for the type on which the current
  implementation is specified.

```rust {all|4-6|8-14|16-18}
struct Foo(i32);

impl Foo {
  fn consume(self) -> Self {
    Self(self.0 + 1)
  }

  fn borrow(&self) -> &i32 {
    &self.0
  }

  fn borrow_mut(&mut self) -> &mut i32 {
    &mut self.0
  }

  fn new() -> Self {
    Self(0)
  }
}
```

---

# Intermission: Impl blocks, the self parameter
The self parameter is called the *receiver*.

* The self parameter is always the first and it always has the type on which it
  was defined
* We never specify the type of the self parameter
* We can optionally prepend `&` or `&mut ` to self to indicate that we take
  a value by reference
* Absence of a self parameter means that the function is an associated function
  instead

```rust
fn main () {
  let mut f = Foo::new();
  println!("{}", f.borrow());
  *f.borrow_mut() = 10;
  let g = f.consume();
  println!("{}", g.borrow());
}
```

---

# `Vec`: storing more of the same
The vector is an array that can grow

```rust
fn main() {
  let arr = [1, 2];
  println!("{:?}", arr);

  let mut nums = Vec::new();
  nums.push(1);
  nums.push(2);
  println!("{:?}", nums);
}
```

* Compare this to the array we previously saw, which has a fixed size
* `Vec` is heap-allocated: `!Copy`: move semantics apply

---

# Put it in a `Box<T>`
That pointer from the stack to the heap, how do we create such a thing?

* Creates data that is stored on the heap
* A `Box<T>` uniquely owns its `T`
* Even if `T` is `Copy`, `Box<T>` is `!Copy`: move semantics apply.

```rust
fn main() {
  // put an integer on the heap
  let boxed_int = Box::new(10);
}
```

<div class="relative left-10px" style="background-color:lightgrey; padding-bottom:10px;">

![Memory Layout](/images/A2-box-in-memory.drawio.svg)

</div>

---

# Vectors and arrays
What if we wanted to write a sum function, we could define one for arrays of
a specific size:

```rust
fn sum(data: &[i64; 10]) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}
```

---

# Vectors and arrays
Or one for just vectors:

```rust
fn sum(data: &Vec<i64>) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}
```

---
layout: two-cols
---

# Slices
What if we want something to work on arrays of any size? Or just parts?

Use slices!
* *Dynamically sized view* into a *contiguous* sequence
* Slices are typed as `[T]`
* must always be behind a reference type, i.e. `&[T]` and `&mut [T]`
  (but also `Box<[T]>` etc)
::right::

<v-click>
<div>

#### `&[T]` is a *fat pointer* :

<div class="relative top-15px" style="background-color:lightgrey; padding: 5px 0;">

![Memory Layout](/images/A2-slice-ptr.drawio.svg)

</div>
</div>
</v-click>

---

# Creating slices
Using a borrow

```rust
fn sum(data: &[i32]) -> i32 { /* ... */ }

fn main() {
  let v = vec![1, 2, 3, 4, 5, 6];
  let total = sum(&v);
  println!("{}", total);
}
```

---

# Creating slices
Using ranges

```rust
fn concat_chars(data: &[char]) -> String { /* ... */ }

fn main() {
  let v = vec!['⭐', '❤️', '🤠', '🐴', '🚃', '🐠'];
  let all = sum(&v[..]);
  let except_first = sum(&v[1..]);
  let except_last = sum(&v[..5]);
  let except_ends = sum(&v[1..5]);
}
```

* The range `start..end` contains all values `x` with `start <= x < end`.

<v-click>
<div>

* Note: you can also use ranges on their own, for example in a for loop:

```rust
fn main() {
  for i in 0..10 {
    println!("{}", i);
  }
}
```
</div>
</v-click>

---

# Practice time!

&nbsp;

A3 exercise description: [workshop.tweede.golf](https://workshop.tweede.golf/A3-advanced-syntax/mod.html)


---
layout: cover
---

# Part 4
Traits & Generics

---
layout: default
---
# The problem

```rust
fn add_u32(l: u32, r: u32) -> u32 { /* -snip- */ }

fn add_i32(l: i32, r: i32) -> i32 { /* -snip- */ }

fn add_f32(l: f32, r: f32) -> f32 { /* -snip- */ }

/* ... */
```

<v-click>
<div>
<strong>We need generic code!</strong>
</div>
</v-click>

<!--
Let's have a look at this Rust module. We'd like to provide functionality for finding the maximum of two numbers, for several distict types. One way to go about it, is to define many similar functions that perform the operation. But there's a number of problems with that:
- What happens if we want to compare other types?
- What happens if we want to compare separate types?
-->

---
layout: default
---
# Generic code

An example
```rust
fn add<T>(lhs: T, rhs: T) -> T { /* - snip - */}
```

<v-click>
<div>
<br/>
Or, in plain English:

- `<T>` = "let `T` be a type"
- `lhs: T` "let `lhs` be of type `T`"
- `-> T` "let `T` be the return type of this function"
</div>
</v-click>
<v-click>
<div>
<br/>
Some open points:

- What can we do with a `T`?
- What should the body be?
</div>
</v-click>

---
layout: default
---
# Bounds on generic code
&nbsp;

We need to provide information to the compiler:
- Tell Rust what `T` can do
- Tell Rust what `T` is accepted
- Tell Rust how `T` implements functionality

---
layout: default
---

# `trait`
&nbsp;

Describe what the type can do
```rust
trait MyAdd {
    fn my_add(&self, other: &Self) -> Self;
}
```

---
layout: default
---
# `impl trait`
&nbsp;

Describe how the type does it

```rust{all|1|2-8}
impl MyAdd for u32 {
    fn my_add(&self, other: &Self) -> Self {
      *self + *other
    }
}
```

---
layout: default
---
# Using a `trait`

```rust{all|1-2|5-6|7-9|10-12}
// Import the type and the trait
use my_mod::{MyAdd}

fn main() {
  let left: u32 = 6;
  let right: u32 = 8;
  // Call trait method
  let result = left.my_add(&right);
  assert_eq!(result, 14);
  // Explicit call
  let result = MyAdd::my_add(&left, &right);
  assert_eq!(result, 14);
}
```

- Trait needs to be in scope
- Call just like a method
- Or by using the explicit associated function syntax

---
layout: default
---
# Trait bounds

```rust{all|1-3,5|5,7-11}
fn add_values<T: MyAdd>(this: &T, other: &T) -> T {
  this.my_add(other)
}

// Or, equivalently

fn add_values<T>(this: &T, other: &T) -> T 
  where T: MyAdd
{
  this.my_add(other)
}
```

Now we've got a *useful* generic function!

English: *"For all types `T` that implement the `MyAdd` `trait`, we define..."*

---
layout: default
---
# Limitations of `MyAdd`
What happens if...

- We want to add two values of different types?
- Addition yields a different type?

---
layout: default
---

# Making `MyAdd` itself generic
&nbsp;

Add an 'Input type' `O`:

```rust{all|1-3|5-9}
trait MyAdd<O> {
    fn my_add(&self, other: &O) -> Self;
}

impl MyAdd<u16> for u32 {
    fn my_add(&self, other: &u16) -> Self {
      *self + (*other as u32)
    }
}
```

We can now add a `u16` to a `u32`.

---
layout: default
---

# Defining output of `MyAdd`

- Addition of two given types always yields in one specific type of output
- Add *associated type* for addition output

```rust{all|2-3|7-9|6-20}
trait MyAdd<O> {
    type Output;
    fn my_add(&self, other: &O) -> Self::Output;
}

impl MyAdd<u16> for u32 {
    type Output = u64;

    fn my_add(&self, other: &u16) -> Self::Output {
      *self as u64 + (*other as u64)
    }
}

impl MyAdd<u32> for u32 {
    type Output = u32;

    fn my_add(&self, other: &u32) -> Self::Output {
      *self + *other
    }
}
```

---
layout: default
---
# `std::ops::Add`
The way `std` does it

```rust{all|1|2-4}
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

- Default type of `Self` for `Rhs`

---
layout: default
---
# `impl std::ops::Add`

```rust
use std::ops::Add;
pub struct BigNumber(u64);

impl Add for BigNumber {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
      BigNumber(self.0 + rhs.0)
  }
}

fn main() {
  // Call `Add::add`
  let res = BigNumber(1).add(BigNumber(2));
}
```

What's the type of `res`?

---
layout: default
---
# `impl std::ops::Add` (2)

```rust
pub struct BigNumber(u64);

impl std::ops::Add<u32> for BigNumber {
  type Output = u128;
  
  fn add(self, rhs: Self) -> Self::Output {
      (self.0 as u128) + (rhs as u128)
  }
}

fn main() {
  let res = BigNumber(1) + 3u32;
}
```

What's the type of `res`?

---
layout: default
---
# Traits: Type Parameter vs. Associated Type

### Type parameter (input type)
*if trait can be implemented for many combinations of types*
```rust
// We can add both a u32 value and a u32 reference to a u32
impl Add<u32> for u32 {/* */}
impl Add<&u32> for u32 {/* */}
```

### Associated type (output type)
*to define a type for a single implementation*
```rust
impl Add<u32> for u32 {
  // Addition of two u32's is always u32
  type Output = u32;
}
```

---
layout: default
---

# `#[derive]` a `trait`

```rust
#[derive(Clone)]
struct Dolly {
  num_legs: u32,
}

fn main() {
  let dolly = Dolly { num_legs: 4 };
  let second_dolly = dolly.clone();
  assert_eq!(dolly.num_legs, second_dolly.num_legs);
}
```

- Some traits are trivial to implement
- Derive to quickly implement a trait
- For `Clone`: derived `impl` calls `clone` on each field 

---
layout: default
---

# Compiling generic functions

```rust
impl MyAdd for i32 {/* - snip - */}
impl MyAdd for f32 {/* - snip - */}

fn add_values<T: MyAdd>(left: &T, right: &T) -> T
{
  left.my_add(right)
}

fn main() {
  let sum_one = add_values(&6, &8);
  assert_eq!(sum_one, 14);
  let sum_two = add_values(&6.5, &7.5);
  println!("Sum two: {}", sum_two); // 14
}
```

Code is <em>monomorphized</em>:
 - Two versions of `add_values` end up in binary
 - Optimized separately and very fast to run (static dispatch)
 - Slow to compile and larger binary

---
layout: default
---

# Practice time!
&nbsp;

A4 exercise description: [workshop.tweede.golf](https://workshop.tweede.golf/A4-traits-generics/mod.html)
