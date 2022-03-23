//! This program compiles to a binary that print "Hello, Rust!" on standard
//! output!
//!
//! It is written as a demo purpose how showing how to use Foreign Function
//! Interface (FFI) in Rust.
//!
//! The Book have sections about FFI in the unsafe chapter "Using extern
//! Functions to Call External Code"
//! https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
//!
//! Currently, this program relies on `.rlib` Rust custom static library format
//! rather than on `libhello.a`.
//!
//! To link `libhello.a` in Rust we should first require a `build.rs` Cargo
//! customization:
//!
//! ```ignore
//! fn main() {
//!     // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
//!     println!("cargo:rustc-link-search=../target/debug");
//! }
//! ```
//!
//! and then to declare the function in an `extern` block:
//!
//! ```ignore
//! #[link(name = "hello", kind = "static")]
//! extern "C" {
//!     fn c_hello(input: *const c_char);
//! }
//! ```

use hello::hello; // Import dumb safe `hello` version

mod c {
    /// Wrapper around an unsafe call to `c_hello` FFI
    #[allow(dead_code)] // I choose not to use it and let it available for demo
    pub fn hello(input: &str) {
        let input = std::ffi::CString::new(input).expect("contain an internal 0 byte");
        // TODO: display `std::ffi::NulError` information in panic message
        unsafe {
            hello::c_hello(input.as_ptr());
        }
    }
}

fn main() {
    hello("Rust"); // This is homolog to c::hello("Rust");
}
