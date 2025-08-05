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
  - like web servers!
- Look and feel* of synchronous code through `async`/`await` syntax

**Well, not perfectly. We'll get to that*

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

Unit 4.3 exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

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

Unit 4.3 exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!

---
layout: default
---

# Requirements change

- how does this app scale?

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

# Final Practice time!

&nbsp;

Unit 4.3 exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!

---
layout: default
---

# State of the `async` art
What you can expect doing `async` Rust

- Blazingly fast applications
- More interaction with advanced language features
- Compatibility issues (re: colored functions)
- Faster evolving ecosystem
- `async fn` in traits stable since Rust 1.75

*But still a work in progress*

---
layout: default
---

# Support of `async`

- Fundamental types and traits are in `std`
- `async`/`await` are native to the language
- Utilities/extensions in `futures` crate
- Async runtimes are third party

Example runtimes: `async-std`, `tokio`, `smol`

---
layout: cover
---
# The `Future` trait
Foundation of async

---
layout: two-cols
---

# A `VerySimpleFuture`
```rust
trait VerySimpleFuture {
    type Output;
    /// Do work and check if task is completed.
    /// Returns [Poll::Ready], containing the
    /// `Output` if task is ready,
    /// [Poll::Pending] if not
    fn poll(&mut self) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```
::right::
<v-click>

```rust
struct VerySimpleAlarm {
    alarm_time: Instant,
}

impl VerySimpleFuture for VerySimpleAlarm {
    type Output = ();

    fn poll(&mut self) -> Poll<()> {
        if Instant::now() >= self.alarm_time {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
```
</v-click>

---
layout: three-slots
---

# Executing `VerySimpleFuture`

::left::

 ```rust
fn main() {
    let mut first_alarm = VerySimpleAlarm {
        alarm_time: Instant::now()
            + Duration::from_secs(3)
    };
    let mut snooze_alarm = VerySimpleAlarm {
        alarm_time: Instant::now()
            + Duration::from_secs(5)
    };

    loop {
        if let Poll::Ready(_) = first_alarm.poll() {
            println!("Beep beep beep");
        }
        if let Poll::Ready(_) = snooze_alarm.poll() {
            println!("You're late for work!")
        }
    }
}
 ```
::right::
<v-click>
<div>

```txt
[pause...]
Beep beep beep
Beep beep beep
[... a few moments later...]
You're late for work!
Beep beep beep
You're late for work!
Beep beep beep
You're late for work!
[...ad infinitum]
```

## It works! 🎉

*Question: How can `VerySimpleFuture` be improved?*
</div>
</v-click>

---
layout: default
---
# Limitation of `VerySimpleAlarm`

- Busy waiting
- How to signal the executor the future is *actually* ready to be polled?

<v-click>
<div>
<br/>

## ⏰ Introduce a Waker

General idea:
- Run some callback to notify executor
- Have executor implement some job queue
</div>
</v-click>

---
layout: default
---
# A `SimpleFuture`

```rust
trait SimpleFuture {
    type Output;

    fn poll(&self, wake: fn()) -> Poll<Self::Output>;
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {             // <-- Does syscall
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);    // <-- Does syscall
            Poll::Pending
        }
    }
}
```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*


---
layout: three-slots
---

# Joining `SimpleFuture`s

::left::

```rust
pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture
    for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*
::right::
```rust
    fn poll(&mut self, wake: fn())
        -> Poll<Self::Output>
    {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take(); // Drop future A
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take(); // Drop future B
            }
        }
        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(()) // Both futures dropped
        } else {
            Poll::Pending // A future is pending
        }
    }
}
```


---
layout: default
---
# And then...

```rust
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }
        self.second.poll(wake)
    }
}
```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*

---
layout: default
---

# `SimpleFuture` takeaways

- Composing `SimpleFuture`s requires no heap allocations
- Composing `SimpleFuture`s requires no deeply nested callbacks

---
layout: default
---

# The `Future` is now!

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

*Question: what stands out to you?*
<v-click>
<div>

- `&mut self` &rarr; `Pin<&mut Self>`: makes `Self` immovable
- `wake: fn()` &rarr; `cx: &mut Context<'_>`: contains a `Waker`

*More on `Pin<&mut Self>` in the [Rust async book](https://rust-lang.github.io/async-book/part-reference/pinning.html)*
</div>
</v-click>

---
layout: cover
---

# `async` and `await`

---
layout: three-slots
---

# Expanding `async`
&nbsp;

*" `Futures` are cool, but why didn't I see them in the web scraper example?"*

<v-click>
<div>
<br/>

`async fn`s and `async` blocks are syntactic sugar generating `Future`s
```rust
async fn foo() -> u8 { 5 }
```
</div>
</v-click>
::left::
<v-click>
<div>

is equivalent to:
```rust
fn foo() -> impl Future<Output=u8> {
    async {
        5
    }
}
```
</div>
</v-click>
::right::
<v-click>
<div>

which is equivalent to:
```rust
fn foo() -> impl Future<Output=u8> {
    /// Create a future that is immediately ready with a value.
    futures::future::ready(5)
}
```
</div>
</v-click>

---
layout: default
---

# Expanding `async` and `await`

```rust
let fut_one = /* ... */;
let fut_two = /* ... */;
async move {            // <-- generated Future takes ownership of referenced variables
    fut_one.await;
    fut_two.await;
}
```
<v-click>
<div>
Generates an opaque type implementing `Future`:

```rust
struct AsyncFuture {
    fut_one: FutOne,
    fut_two: FutTwo,
    state: State,
}
enum State {
    AwaitingFutOne,
    AwaitingFutTwo,
    Done,
}
```

*This and the following is not the actually generated code, but it's a good mental model*
</div>
</v-click>


---
layout: default
---

# Expanding `async` and `await` (2)

```rust
impl Future for AsyncFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            match self.state {
                State::AwaitingFutOne => match self.fut_one.poll(/* - snip - */) {
                    Poll::Ready(()) => self.state = State::AwaitingFutTwo,
                    Poll::Pending => return Poll::Pending,
                }
                State::AwaitingFutTwo => match self.fut_two.poll(/* - snip - */) {
                    Poll::Ready(()) => self.state = State::Done,
                    Poll::Pending => return Poll::Pending,
                }
                State::Done => return Poll::Ready(()),
            }
        }
    }
}
```

Kind of looks like `AndThenFut`!

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)*

---
layout: default
---

# `async`/`await` expansion takeaways

- Rust generates state machines out of `async` blocks that implement `Future`
- You can `await` `Future`s
- Every `await` point introduces a new state
- Generated code may become very complex, but original is easy to follow

---
layout: cover
---

# Running `Future`s

---
layout: default
---

# What's an `async` Runtime do?

- Spawn `Future`s
- Keep track of pending `Future`s
- Call `Future::poll` on each `Future` that can make progress
- Poll `Future`s on `Waker::wake` calls

Nice to have:
- Poll `Future`s on multiple threads
- Abstract over I/O

*Crates depending on different runtime I/O abstractions be incompatible!*
---
layout: default
---

# Many Runtime flavors

- [`smol`](https://github.com/smol-rs/smol): Small
- [`async-std`](https://async.rs/): API resembles `std`
- [`tokio`](https://tokio.rs): Very commonly used
- [`embassy`](https://embassy.dev/): Embedded
- Create your own?

*Note: crates may depend on a specific runtime!*

---
layout: default
---

# Showcase: Tokio

```rust
/// Set up a tokio Runtime and spawn the Future returned by `main`
#[tokio::main]
async fn main() {
    do_stuff().await;
}
```

It does stuff!

---
layout: default
---

# A simple TCP server

```rust
use tokio::net::{TcpListener, TcpStream};

/// Read a line, and reply with that line!
async fn handle_connection(socket: TcpStream) -> anyhow::Result<()> {
    let mut stream = BufReader::new(socket);
    let mut name = String::new();
    stream.read_line(&mut name).await?;

    stream.write_all(format!("Hello, {name}!").as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        handle_connection(socket).await?;
    }
}
```

---
layout: default
---
# It works!

```bash
$ echo -e Ferris | nc localhost 6379
Hello Ferris!⏎
```

*Question: But does it scale?*

<v-click>
<div>
Nope! Only one request at a time!
</div>
</v-click>

---
layout: default
---

# Spawning tasks is cheap!
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        handle_connection(socket).await?;
    }
}
```
becomes:
```rust
async fn main() -> Result<()> {
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async {
            handle_connection(socket).await?;
            Ok::<_, anyhow::Error>(())
        });
    }
}
```

---
layout: cover
---

# Async patterns


---
layout: default
---
# Task

A task is a light weight, non-blocking unit of execution.

> A task is similar to an OS thread, but rather than being managed by the OS scheduler, they are managed by the Tokio runtime. Another name for this general pattern is green threads.

https://docs.rs/tokio/latest/tokio/task/index.html

---
layout: default
---
# Stream

A stream is the async equivalent of an iterator.

because the `Stream` or `AsyncIterator` trait is not stable, these apis are not used the often today.

---
layout: default
---

# Practice time!

&nbsp;

Unit 4.3 exercise description: [training.tweede.golf](https://training.tweede.golf/asynchronous-multitasking.html)

*Don't forget to* `git pull`!
