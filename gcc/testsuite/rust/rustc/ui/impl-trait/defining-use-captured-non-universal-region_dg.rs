// This was an ICE. See #110726.

//@ revisions: statik infer fixed
//@ [fixed] check-pass
#![allow(unconditional_recursion)]

fn foo<'a>() -> impl Sized + 'a {
    #[cfg(statik)]
    let i: i32 = foo::<'static>();
// { dg-error "" "" { target *-*-* } .-1 }

    #[cfg(infer)]
    let i: i32 = foo::<'_>();
// { dg-error "" "" { target *-*-* } .-1 }

    #[cfg(fixed)]
    let i: i32 = foo::<'a>();

    i
}

fn main() {}

