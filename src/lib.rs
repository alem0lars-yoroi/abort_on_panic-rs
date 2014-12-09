//! To use this macro, you'll need to include the following declarations
//! at the top level of your crate.
//!
//! ```ignore
//! #![feature(phase)]
//! #[phase(plugin, link)] extern crate abort_on_panic;
//! ```
//!
//! Then you can invoke it as follows:
//!
//! ```ignore
//! let result = abort_on_panic!({ "value" });
//! assert_eq!("value", result);
//!
//! abort_on_panic("cannot panic inside FFI callbacks", {
//!   // ...
//! });
//! ```
#![feature(macro_rules)]

use std::intrinsics::abort;
use std::io::stderr;
use std::task::failing;

/// Once this object is created, it can only be destroyed in an orderly
/// fashion.  Attempting to clean it up from a panic handler will abort the
/// process.
pub struct PanicGuard {
    // We hope that this will be optimized heavily.
    message: Option<&'static str>
}

impl PanicGuard {
    /// Create a panic guard with a generic message.
    pub fn new() -> PanicGuard { PanicGuard{message: None} }

    /// Create a panic guard with a custom message.
    pub fn with_message(message: &'static str) -> PanicGuard {
        PanicGuard{message: Some(message)}
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        if failing() {
            let msg = self.message.unwrap_or("cannot unwind past stack frame");
            let _ = writeln!(&mut stderr(), "{} at {}:{}",
                             msg, file!(), line!());
            unsafe { abort(); }
        }
    }
}

/// Run a block of code, aborting the entire process if it tries to panic.
#[macro_export]
macro_rules! abort_on_panic {
    ($message:expr, $body:block) => {
        {
            let _guard = ::abort_on_panic::PanicGuard::with_message($message);
            $body
        }
    };

    ($body:block) => {
        {
            let _guard = ::abort_on_panic::PanicGuard::new();
            $body
        }
    };
}
