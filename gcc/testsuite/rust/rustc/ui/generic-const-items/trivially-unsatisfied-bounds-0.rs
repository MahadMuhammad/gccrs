#![feature(generic_const_items, trivial_bounds)]
#![allow(incomplete_features)]

// Ensure that we check if trivial bounds on const items hold or not.

const UNUSABLE: () = () // { dg-error ".E0080." "" { target *-*-* } }
where
    String: Copy;

fn main() {
    let _ = UNUSABLE; // { dg-error ".E0277." "" { target *-*-* } }
}

