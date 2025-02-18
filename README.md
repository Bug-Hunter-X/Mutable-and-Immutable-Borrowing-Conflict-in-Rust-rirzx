# Mutable and Immutable Borrowing Conflict in Rust

This example showcases a common error in Rust related to mutable and immutable borrowing.  Rust's strict borrowing rules prevent data races and ensure memory safety.  This example demonstrates how violating these rules leads to a compile-time error.

## The Bug

The `bug.rs` file contains code that attempts to have both a mutable and an immutable borrow of the same variable simultaneously. This violates Rust's borrowing rules.

## The Solution

The `bugSolution.rs` file presents a corrected version, demonstrating how to resolve the conflict by either removing the immutable borrow or rearranging the code to avoid overlapping borrows.