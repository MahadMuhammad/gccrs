#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]

pub const fn test(_: &Type) {
    const fn takes_borrow(_: &Type) {}

    let local = Type;
    become takes_borrow(&local);
// { dg-error ".E0597." "" { target *-*-* } .-1 }
}

struct Type;

fn main() {}

