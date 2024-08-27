use std::mem::size_of;

struct Foo<'s> { // { dg-error ".E0392." "" { target *-*-* } }
    array: [(); size_of::<&Self>()],
// { dg-error "" "" { target *-*-* } .-1 }
}

// The below is taken from https://github.com/rust-lang/rust/issues/66152#issuecomment-550275017
// as the root cause seems the same.

const fn foo<T>() -> usize {
    0
}

struct Bar<'a> { // { dg-error ".E0392." "" { target *-*-* } }
    beta: [(); foo::<&'a ()>()], // { dg-error "" "" { target *-*-* } }
}

fn main() {}

