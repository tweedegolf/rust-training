use std::{fmt::Debug, iter::Peekable, str::Split};

use format::ServerToDevice;

type ChunkIter<'a> = Peekable<Split<'a, char>>;
use commands::*;

/// Trait that defines a command.
pub trait Command: Debug {
    /// Parse a command
    fn parse(chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
    where
        Self: Sized;

    /// Build up a ServerToDevice message given the command's state
    fn build_message(&self) -> ServerToDevice;

    /// Helper method to easily put this command on the heap
    fn boxed(self) -> Box<dyn Command>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

#[derive(Debug)]
pub enum ParseError {
    CommandNotFound,
    InvalidArgs,
}

pub struct CommandParser;

impl CommandParser {
    pub fn parse(cmd: &str) -> Result<Box<dyn Command>, ParseError> {
        let chunks = cmd.split(' ').peekable();

        fn parse_next<'c, C: Command>(
            chunks: ChunkIter<'c>,
        ) -> impl FnOnce(ParseError) -> Result<Box<dyn Command>, ParseError> + 'c {
            move |e: ParseError| match e {
                ParseError::CommandNotFound => C::parse(chunks),
                r => Err(r),
            }
        }

        // Iterators are stateful, so we need to clone
        LedStatus::parse(chunks.clone())
            // TODO parse other commands, chaining calls like this:
            .or_else(parse_next::<SayHello>(chunks.clone()))
    }
}

mod commands {
    use format::ServerToDevice;

    use super::{
        ChunkIter, Command,
        ParseError::{self, *},
    };

    /// LedStatus command. Used to tell
    /// the device to set a led state
    #[derive(Debug)]
    pub struct LedStatus {
        led_no: u8,
        on: bool,
    }

    impl Command for LedStatus {
        fn parse(mut chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
        where
            Self: Sized,
        {
            // Get first word
            let cmd = chunks.next();
            // Get second word and parse as byte
            let arg1: Option<u8> = chunks.next().map(|a| a.parse().ok()).flatten();
            // Get third word and parse as bool: "on" -> true, "off" -> false
            let arg2: Option<bool> = chunks
                .next()
                .map(|a| match a {
                    "on" => Some(true),
                    "off" => Some(false),
                    _ => None,
                })
                .flatten();

            match (cmd, arg1, arg2) {
                // Accepted if first word equals "led",
                // arg1 is between 1 and 4 (inclusive)
                // and arg2 was parsed successfully
                (Some("led"), Some(led_no @ 1..=4), Some(on)) => Ok(Self { led_no, on }.boxed()),
                // Invalid arguments if first word equeals "led",
                // but the rest does not match
                (Some("led"), _, _) => Err(InvalidArgs),
                // Not found otherwise, try to parse another command
                _ => Err(CommandNotFound),
            }
        }

        fn build_message(&self) -> ServerToDevice {
            ServerToDevice {
                set_led_status: Some((self.led_no, self.on)),
                ..ServerToDevice::default()
            }
        }
    }

    /// SayHello command. Tells the
    /// device to be polite and say hi back
    #[derive(Debug)]
    pub struct SayHello;

    impl Command for SayHello {
        fn parse(mut chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
        where
            Self: Sized,
        {
            // Get first word
            let cmd = chunks.next();

            match cmd {
                // Accepted if the first word was "hello"
                Some("hello") => Ok(Self.boxed()),
                // Not found otherwise
                _ => Err(CommandNotFound),
            }
        }

        fn build_message(&self) -> ServerToDevice {
            ServerToDevice {
                say_hello: true,
                ..ServerToDevice::default()
            }
        }
    }

    // TODO add your own commands here
}
