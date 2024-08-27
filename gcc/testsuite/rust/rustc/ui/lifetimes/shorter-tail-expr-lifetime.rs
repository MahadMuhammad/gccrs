//@ revisions: edition2021 edition2024
//@ [edition2024] compile-flags: -Zunstable-options
// { dg-additional-options "-frust-edition= 2024" }
//@ [edition2024] run-pass

#![cfg_attr(edition2024, feature(shorter_tail_lifetimes))]

fn f() -> usize {
    let c = std::cell::RefCell::new("..");
    c.borrow().len() // { dg-error "" "" { target *-*-* } }
}

fn main() {
    let _ = f();
}

