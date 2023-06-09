use std::{env, path::PathBuf};

fn main() {
    // Rerun this build script only if eiter `tweetnacl.h`
    // or `tweetnacl.c` changes, which would affect our bindings
    println!("cargo:rerun-if-changed=tweetnacl.h");
    println!("cargo:rerun-if-changed=tweetnacl.c");
    println!("cargo:rerun-if-changed=build.rs");

    // Taken from The Bindgen User Guide (https://rust-lang.github.io/rust-bindgen/tutorial-3.html)
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("tweetnacl.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Format bindings with rustfmt
        .formatter(bindgen::Formatter::Rustfmt)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile C code and setup linking
    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("tweetnacl.c")
        .compile("tweetnacl"); // outputs `libtweetnacl.a`

    // Print the location the bindings were written to
    // when compiling this crate.
    // We can only do that in a warning from here, which I
    // admit is a bit hacky. But oh well.
    println!(
        "cargo:warning=bindings.rs location: {}",
        out_path
            .join("bindings.rs")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    )
}
