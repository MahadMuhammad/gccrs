// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

fn main() {
    let _ = ||{}();
// { dg-error ".E0618." "" { target *-*-* } .-1 }

    let _ = async ||{}();
// { dg-error ".E0618." "" { target *-*-* } .-1 }
}

