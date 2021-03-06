[![Build Status](https://travis-ci.org/emk/abort_on_panic-rs.svg)](https://travis-ci.org/emk/abort_on_panic-rs) [![Latest version](https://img.shields.io/crates/v/abort_on_panic.svg)](https://crates.io/crates/abort_on_panic) [![License](https://img.shields.io/crates/l/abort_on_panic.svg)](https://crates.io/crates/abort_on_panic)

[Documentation](http://emk.github.io/abort_on_panic-rs/abort_on_panic/index.html).<br>
[StackOverflow question](http://stackoverflow.com/questions/27384824/catching-panic-when-rust-called-from-c-ffi-without-spawning-threads) looking for alternatives.

When calling Rust code from C, it's unsafe to call `panic!`.  Doing so may
cause unsafe behavior.  But when calling user-defined functions, we
sometimes need to enforce these rules.

To use this library, add the following to your `Cargo.toml` file:

```
[dependencies]
abort_on_panic = "*"
```

You can then automatically `abort` the process when a `panic!` occurs in a
inconvenient location:

```rust
#![feature(phase)]
#[phase(plugin, link)] extern crate abort_on_panic;

pub fn main() {
    let result = abort_on_panic!({
        "value"
    });
    assert_eq!("value", result);

    fn oops() {
        panic!("Uh oh.");
    }
    
    abort_on_panic!({
        oops();
    });
}
```

### Motivation

I'm working on a
[Rust wrapper for the Duktape JavaScript interpreter][duktape-rs]. In a
normal use case, the call stack will look like this:

1. Rust: Arbitrary application code.
2. Rust: My library wrapper.
3. C: The Duktape interpreter.
4. Rust: My Rust code.
5. Rust: Arbitrary callbacks into application code.

What happens if (5) calls `panic!`? According to various Rust developers on IRC, attempting to `panic!` from inside non-Rust callframes like (3) may result in undefined behavior.

But according the Rust documentation, the only way to catch a `panic!` is
using [`std::task::try`][task::try], which spawns an extra thread. There's
also [`rustrt::unwind::try`][unwind::try], which cannot be nested twice
within a single thread, among other restrictions.

One solution, proposed by Benjamin Herr, is to abort the process if the
code in (5) panics.  Sure, this is less than ideal, but aborting is better
than silently corrupting memory and continuing.  So this library goes with
`abort`.  Are there better alternatives?

[duktape-rs]: https://github.com/emk/duktape-rs
[task::try]: http://doc.rust-lang.org/std/task/fn.try.html
[unwind::try]: http://doc.rust-lang.org/rustrt/unwind/fn.try.html

### Credits & license

The original idea for this code came from Benjamin Herr.  Many thanks!

This code is placed into the public domain under the terms described by the
Unlicense.
