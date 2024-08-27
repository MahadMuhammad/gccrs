//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition= 2021" }
// { dg-additional-options "-frust-edition= 2024" }
//@ [edition2024] compile-flags: -Zunstable-options
//@ [edition2024] check-pass

#![cfg_attr(edition2024, feature(shorter_tail_lifetimes))]

fn main() {
    let cell = std::cell::RefCell::new(0u8);

    if let Ok(mut byte) = cell.try_borrow_mut() {
// { dg-error "" "" { target *-*-* } .-1 }
        *byte = 1;
    }
}

