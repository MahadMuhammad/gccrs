//@ aux-build:issue-119463-extern.rs

extern crate issue_119463_extern;

struct S;

impl issue_119463_extern::PrivateTrait for S {
// { dg-error ".E0603." "" { target *-*-* } .-1 }
    const FOO: usize = 1;

    fn nonexistent() {}
// { dg-error ".E0407." "" { target *-*-* } .-1 }
}

fn main() {}

