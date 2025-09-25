---
theme: tweedegolf
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 4.3: Asynchronous Multitasking"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 4.3: Asynchronous Multitasking"
routerMode: hash
---

# Rust programming

Module 4: Multitasking

## Unit 3

Asynchronous Multitasking

---
layout: default
---
# Recap: Concurrency vs. Parallelism

| **Concurrency**                                                                                                          | **Parallelism**                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Interleaves work                                                                                                         | Parallelizes work                                                                                                                                                      |
| 1 or more cores                                                                                                          | 2 or more cores                                                                                                                                                        |
| Waiting for events                                                                                                       | Waiting for computation                                                                                                                                                |
| <img src="https://tienda.bricogeek.com/6417-thickbox_default/sparkfun-thing-plus-esp32-wroom.jpg" class="h-40 center" /> | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d3/IBM_Blue_Gene_P_supercomputer.jpg/1920px-IBM_Blue_Gene_P_supercomputer.jpg" class="h-40 center" /> |

Today, we're focusing on concurrency: _asynchronous programming_

---
layout: default
---

# What's async?

- Concurrent programming model
- Very suitable for running a large number of I/O bound tasks
  - like web servers
  - or microcontrollers
- Look and feel of synchronous code through `async`/`await` syntax

---
layout: default
---

# Async vs OS Threads

|                      | <span style="color: red">**Async**</span> | <span style="color: blue">**OS Threads**</span> |
| -------------------- | ----------------------------------------- | ----------------------------------------------- |
| Spawning & switching | Cheap                                     | Expensive                                       |
| Blocking is ok       | No                                        | Yes                                             |
| Usage                | I/O bound tasks (web servers)             | CPU-bound tasks (Number crunching)              |
| Reuse sync code      | No                                        | Yes                                             |

[What Color is Your Function? ](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)

---
layout: cover
---

# Async in Rust


---
layout: default
---

# From sync to async

```rust
use std::net::UdpSocket;

// <nothing>
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    let mut buf = [0; 10];
    let (bytes_read, src) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..bytes_read];
    buf.reverse();
    socket.send_to(buf, &src)?;

    Ok(())
}
```

---
layout: default
---

# From sync to async

```rust
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254").await?;

    let mut buf = [0; 10];
    let (bytes_read, src) = socket.recv_from(&mut buf).await?;

    let buf = &mut buf[..bytes_read];
    buf.reverse();
    socket.send_to(buf, &src).await?;

    Ok(())
}
```

---
layout: default
---

# Practice time!

&nbsp;

Unit 4.3.1 exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!

---
layout: default
---

# Async in Rust

Goals:

- `async` is zero-cost
- `async` runs anywhere (desktop, embedded, wasm, etc.)

---
layout: default
---

# Async in Rust

The central concept is the "Future"

- a Future resolves to a value at some point in the future
- conceptually similar to JS `Promise` or C# `Task`

---
layout: default
---

# Async in Rust

Rust encodes ideas as types: `Future` is a trait!

```rust
pub trait Future {
    type Output;

    fn poll(/* ... */) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

- `Future`s are inert: they don't do anything unless polled
- you rarely impl the future trait in user code
- you rarely explicitly call `.poll()` in user code

---
layout: default
---

# Runtimes

Runtimes take care of `.poll()`ing futures

- `tokio` is the most mature runtime for desktop applications
- `embassy` is an embedded runtime for embedded systems

and there are some other options (and in theory you can write your own)

---
layout: default
---

# async blocks & syntactic sugar

- `async fn` is syntactic sugar for a function that returns a future
- an `async { /* ... */ }` turns that block into a future
- the compiler builds a state machine and `impl Future` for that state machine

```rust
use tokio::net::UdpSocket;

#[tokio::main]
fn main() -> impl std::future::Future<Output = std::io::Result<()>> {
    async {
        let socket = UdpSocket::bind("127.0.0.1:34254").await?;

        let mut buf = [0; 10];
        let (bytes_read, src) = socket.recv_from(&mut buf).await?;

        let buf = &mut buf[..bytes_read];
        buf.reverse();
        socket.send_to(buf, &src).await?;

        Ok(())
    }
}
```

---
layout: cover
---

# Dependencies

---
layout: default
---

# Cargo.toml

```toml
# Cargo.toml

[package]
name = "data-sink"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.116"
anyhow = "1.0.82"
csv = "1.3.0"
```

add dependencies from the command line with

```sh
cargo add csv
```

---
layout: default
---

# Json with serde

- https://docs.rs/serde_json

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

---
layout: default
---

# Logging with tracing

- https://docs.rs/tracing

```rust
info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
```

---
layout: default
---

# Error handling with anyhow

- https://docs.rs/anyhow
- in application code
- for libraries, use `thiserror`

```rust
fn get_cluster_info() -> anyhow::Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

---
layout: default
---

# Practice time!

&nbsp;

Unit 4.3.2A exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!

---
layout: default
---

# Requirements change

- Sensors send data too often
- Cloud bill is too high
- Management: Send measurements only every 10s

---
layout: default
---
# Async combinators: select

https://docs.rs/tokio/latest/tokio/macro.select.html

parallel composition: completes when any future completes


```rust
async fn with_timeout<T, F: Future<Output = T>>(f: F, duration: Duration) -> Option<T> {
    select! {
        _ = sleep(duration) => None,
        result = f => Some(result),
    }
}
```

the other futures are **cancelled**

---
layout: default
---
# Cancel safety

A future is **cancel-safe** when partially completing it is observationally equivalent to not running it at all

```rust
select! {
    _ = sleep(duration) => None,
    n = tokio::io::AsyncRead::read(&mut reader, &mut buf) => Some(n),
}
```

```rust
select! {
    _ = sleep(duration) => None,
    result = tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut buf) => Some(result),
}
```


---
layout: default
---
# Async combinators: join

sequential composition: awaits till all futures complete

```rust
let (res1, res2) = join!(fut1, fut2);
let (res1, res2, res3) = join!(fut1, fut2, fut3);

match try_join!(fut1, fut2) {
    Err(first_to_error) => ...
    Ok((ok1, ok2)) => ...
}
```

---
layout: default
---

# Blocking computation

Issuing a blocking call in a future is problematic: other futures may not be driven forward

```rust
use tokio::task;

// Initial input
let mut v = "Hello, ".to_string();
let res = task::spawn_blocking(move || {
    // Stand-in for compute-heavy work or using synchronous APIs
    v.push_str("world");
    // Pass ownership of the value back to the asynchronous context
    v
}).await?;

// `res` is the value returned from the thread
assert_eq!(res.as_str(), "Hello, world");
```

- spawns a background thread to perform the computation
words you should know

---
layout: default
---

# Re-defining references

- `&T`: (possibly) shared reference
- `&mut T`: exclusive reference


for safe mutation, we need exclusive *access*, which we can get in multiple ways:

- we have an exclusive reference to the value
- we own the value (we can exclusively borrow from ourselves)
- access is inherently exclusive (atomic operations)

---
layout: default
---

# Moving ownership between threads

- Some values should never be shared or moved between threads

The `Send` and `Sync` marker traits enforce this:

```rust
pub unsafe auto trait Send { /* no method */ }
pub unsafe auto trait Sync { /* no method */ }
```

- `Send`: A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread
- `Sync`: A type is Sync if it can be shared with another thread. In other words, a type T is Sync if and only if a shared reference to that type `&T` is Send


---
layout: default
---

# `Send`

- A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread


```rust
impl<T: ?Sized> !Send for MutexGuard<'_, T>
impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T>
```

- On certain OS's, only the thread that locked a mutex may unlock it again!
---
layout: default
---

# MPSC: many producer single consumer

```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::scope(|s| {
        for (i, tx) in std::iter::repeat(tx).take(10).enumerate() {
            s.spawn(move || { tx.send(i).unwrap(); });
        }

        s.spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("{msg}");
            }
        });
    });
}
```

where the `Receiver` is:

```rust
impl<T: Send> Send for Receiver<T>
impl<T> !Sync for Receiver<T>
```

---
layout: default
---

# Words you should know: Runtime, Executor & Reactor

![](https://tweedegolf.nl/images/asynch-runtime-main.png)

---
layout: default
---

# Words you should know: Task

A future that is managed by the runtime

- e.g. you have explicitly `tokio::task::spawn`ed it

---
layout: default
---

# Words you should know: Waker & Context

- Used to signal to the runtime: this future can now make progress again
- Triggered by some OS/hardware trigger

---
layout: default
---

# Words you should know: `Pin`

- Guarantees that a value does not move in memory
- This is required for futures because they may be self-referential
- Moving such values invalidates pointers/references

---
layout: default
---

# Final Practice time!

&nbsp;

Unit 4.3.2B exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!
