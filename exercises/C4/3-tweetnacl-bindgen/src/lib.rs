/// Re-export everything from the `bindings` module
pub use bindings::*;

/// Separate module so warnings are only suppressed for the generated bindings
mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
