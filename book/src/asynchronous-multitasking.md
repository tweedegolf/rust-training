# Unit 4.3 - Asynchronous Multitasking

<a href="/slides/4_3-asynchronous-multitasking/" target="_blank">Slides</a>

## Exercise 4.3.1: Async Channels

Channels are a very useful way to communicate between threads and `async` tasks. They allow for decoupling your application into many tasks. You'll see how that can come in nicely in exercise E.2. In this exercise, you'll implement two variants: a oneshot channel and a multi-producer-single-consumer (MPSC) channel. If you're up for a challenge, you can write a broadcast channel as well.

### 4.3.1.A MPSC channel ⭐⭐
A multi-producer-single-consumer (MPSC) channel is a channel that allows for multiple `Sender`s to send many messages to a single `Receiver`.

Open `exercises/4-multitasking/3-asynchronous-multitasking/1-async-channels` in your editor. You'll find the scaffolding code there. For part A, you'll work in `src/mpsc.rs`. Fix the `todo!`s in that file in order to make the test pass. To test, run:

```bash
cargo test -- mpsc
```

If your tests are stuck, probably either your implementation does not use the `Waker` correctly, or it returns `Poll::Pending` where it shouldn't.

### 4.3.1.B Oneshot channel ⭐⭐⭐
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

### 4.3.1.C Broadcast channel (bonus) ⭐⭐⭐⭐
A Broadcast channel is a channel that supports multiple senders and receivers. Each message that is sent by any of the senders, is received by every receiver. Therefore, the implemenentation has to hold on to messages until they have been sent to every receiver that has not yet been dropped. This furthermore implies that the message shoud be cloned upon broadcasting.

For this bonus exercise, we provide no scaffolding. Take your inspiration from the `mpsc` and `oneshot` modules, and implement a `broadcast` module yourself.
## Exercise 4.3.2: Async Chat

In this exercise, you'll write a simple chat server and client based on [Tokio](https://docs.rs/tokio). Open `exercises/4-multitasking/3-asynchronous-multitasking/2-async-chat` in your editor. The project contains a `lib.rs` file, in which a type `Message` resides. This `Message` defines the data the chat server and clients use to communicate.

### 4.3.2.A Server ⭐⭐⭐
The chat server, which resides in `src/bin/server.rs` listens for incoming TCP connections on port 8000, and spawns two tasks (futures):

- `handle_incoming`: reads lines coming in from the TCP connection. It reads the username the client provides, and broadcasts incoming `Messages`, possibly after some modification.
- `handle_outgoing`: sends messages that were broadcasted by the `handle_incoming` tasks to the client over TCP.

Both `handle_incoming` and `handle_outgoing` contain a number to `todo`s. Fix them.

To start the server, run

```bash
cargo run --bin server
```

### 4.3.2.B Client ⭐⭐
The chat client, residing in `src/bin/client.rs` contains some todo's as well. Fix them to allow for registration and sending `Message`s to the server.

To start the client, run

```bash
cargo run --bin client
```

If everything works well, you should be able to run multiple clients and see messages sent from each client in every other.
