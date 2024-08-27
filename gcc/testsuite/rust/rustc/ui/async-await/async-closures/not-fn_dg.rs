// { dg-additional-options "-frust-edition=2021" }

// FIXME(async_closures): This needs a better error message!

#![feature(async_closure)]

fn main() {
    fn needs_fn<T>(_: impl FnMut() -> T) {}

    let mut x = 1;
    needs_fn(async || {
// { dg-error "" "" { target *-*-* } .-1 }
        x += 1;
    });
}

