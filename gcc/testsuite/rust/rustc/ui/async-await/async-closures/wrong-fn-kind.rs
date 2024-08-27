// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

fn needs_async_fn(_: impl async Fn()) {}

fn a() {
    let mut x = 1;
    needs_async_fn(async || {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        x += 1;
    });
}

fn b() {
    let x = String::new();
    needs_async_fn(move || async move {
// { dg-error ".E0525." "" { target *-*-* } .-1 }
        println!("{x}");
    });
}

fn main() {}

