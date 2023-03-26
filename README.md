Learning to use `fork`
======================

In standard library there is no `fork` function as it is considered unsafe.
For more info see
[here](https://internals.rust-lang.org/t/why-no-fork-in-std-process/13770)

`libc` has the [fork](https://docs.rs/libc/latest/libc/fn.fork.html) function
from C that is unsafe.

When it comes to `exec` the Rust eqvivalent is `std::process::Command` but in
the Rust way.
