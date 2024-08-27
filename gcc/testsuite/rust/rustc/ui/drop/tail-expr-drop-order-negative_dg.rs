//@ revisions: edition2021 edition2024
//@ [edition2024] compile-flags: -Zunstable-options
// { dg-additional-options "-frust-edition= 2024" }
//@ [edition2021] check-pass

#![feature(shorter_tail_lifetimes)]

fn why_would_you_do_this() -> bool {
    let mut x = None;
    // Make a temporary `RefCell` and put a `Ref` that borrows it in `x`.
    x.replace(std::cell::RefCell::new(123).borrow()).is_some()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    why_would_you_do_this();
}

