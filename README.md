# Rust Mutable vs Immutable References

This repository demonstrates a common error in Rust related to mutable and immutable references.  The `bug.rs` file contains code that attempts to modify a value via an immutable reference, resulting in a compile-time error.  `bugSolution.rs` shows how to correct this by using mutable references appropriately.