# Unit 8.4 - The Embassy Framework

<a href="/rust-training/slides/8_4-embassy-framework/" target="_blank">Slides</a>

## Exercise 8.4.1: Embassy project

This exercise is a bit bigger than you're used to: we're going to do a custom project using Embassy and the micro:bit V2. Work together in teams of 2 to 3 people during this exercise.

We have set up a crate for you to work on, with a simpl blinky application. You're free to adapt anything about it, it's just meant as a starting point. The crate resides in `exercises/8-embedded/4-embassy-framework/1-embassy-project`. Try it out to ensure the intial setup works.

You can choose from any of below projects:

- Build a **snake game** that uses the LED matrix as game display. You can use the buttons as input, or use the accelerometer or magnetometer. Or a combination.
- Create a **light banner** that reads text from UART, using the virtual COM port that is exposed by the interface MCU and shows it on the LED matrix.
- Create an **audio recorder** that reads samples from the on-board microphone and replays it using the on-board speaker. Additionally, you can show and FFT chart or a volume meter on the display
- Write a **driver for the capacative touch sensor**
- Build a **button masher multiplayer game**. Who is the fastest button presser? Either have it work with the two on-board buttons, or connect multiple boards together somehow. Show the score on the LED matrix.
