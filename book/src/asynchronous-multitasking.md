# Unit 4.3 - Asynchronous Multitasking

<a href="/slides/4_3-asynchronous-multitasking/" target="_blank">Slides</a>

## Exercise 4.3.1: From sync to async

Synchronous and asynchronous Rust code does not look too different from each other. In this excercise we will turn a synchronous Rust TCP echo server into an asynchronous one.

Open `exercises/4-multitasking/3-asynchronous-multitasking/1-sync-to-async` in your editor. Follow the steps in `main.rs` to first test the program works, then convert it to `async`, and then test it still works.

## Exercise 4.3.2: Measurement Data Sink

In this scenario we have a set of IoT sensors that measure air quality in different rooms. They send the data via a TCP socket to a server. The server aggregates the data per room and writes the data to CSV file. The functionality is currently implemented in a synchronous way. Your task is to make the server code `async`.

Open `exercises/4-multitasking/3-asynchronous-multitasking/2-measurement-data-sink` in your editor.

In two different terminals run:

```bash
cargo run --bin server
```

and

```bash
cargo run --bin sensor-nodes
```

You should see regular log messages about received measurements. Every 60 seconds new lines should be appended to `database.csv`.

### Exercise 4.3.2A: `async`ify

Then address the `TODO:` comments in `src/bin/server.rs`. Check that running the application still works as before.

### Exercise 4.3.2B: Requirements change

Run the clients with an interval of 10 seconds like this:

```bash
cargo run --bin sensor-nodes -- -i 10s
```

Investigate and address upcoming bugs.

## Exercise 4.3.3: Async Channels

Channels are a very useful way to communicate between threads and `async` tasks. They allow for decoupling your application into many tasks. You'll see how that can come in nicely in exercise E.2. In this exercise, you'll implement two variants: a oneshot channel and a multi-producer-single-consumer (MPSC) channel. If you're up for a challenge, you can write a broadcast channel as well.

### 4.3.3.A MPSC channel ⭐⭐
A multi-producer-single-consumer (MPSC) channel is a channel that allows for multiple `Sender`s to send many messages to a single `Receiver`.

Open `exercises/4-multitasking/3-asynchronous-multitasking/3-async-channels` in your editor. You'll find the scaffolding code there. For part A, you'll work in `src/mpsc.rs`. Fix the `todo!`s in that file in order to make the test pass. To test, run:

```bash
cargo test -- mpsc
```

If your tests are stuck, probably either your implementation does not use the `Waker` correctly, or it returns `Poll::Pending` where it shouldn't.

### 4.3.3.B Oneshot channel ⭐⭐⭐
A oneshot is a channel that allows for one `Sender` to send exactly one message to a single `Receiver`.

For part B, you'll work in `src/broadcast.rs`. This time, you'll have to do more yourself. Intended behavior:

- `Receiver` implements `Future`. It returns `Poll::Ready(Ok(T))` if `inner.data` is `Some(T)`, `Poll::Pending` if `inner.data` is `None`, and `Poll::Ready(Err(Error::SenderDropped))` if the `Sender` was dropped.
- `Receiver::poll` replaces `inner.waker` with the one from the `Context`.
- `Sender` consumes `self` on send, allowing the it to be used no more than once. Sending sets `inner.data` to `Some(T)`. It returns `Err(Error::ReceiverDropped(T))` if the `Receiver` was dropped before sending.
- `Sender::send` wakes `inner.waker` after putting the data in `inner.data`
- Once the `Sender` is dropped, it marks itself dropped with `inner`
- Once the `Receiver` is dropped, it marks itself dropped with `inner`
- Upon succesfully sending the message, the consumed `Sender` is not marked as dropped. Instead `std::mem::forget` is used to avoid running the destructor.

To test, run:
```bash
cargo test -- broadcast
```

### 4.3.3.C Broadcast channel (bonus) ⭐⭐⭐⭐
A Broadcast channel is a channel that supports multiple senders and receivers. Each message that is sent by any of the senders, is received by every receiver. Therefore, the implemenentation has to hold on to messages until they have been sent to every receiver that has not yet been dropped. This furthermore implies that the message shoud be cloned upon broadcasting.

For this bonus exercise, we provide no scaffolding. Take your inspiration from the `mpsc` and `oneshot` modules, and implement a `broadcast` module yourself.
