// { dg-additional-options "-frust-edition= 2021" }
//@ run-rustfix

#![allow(unused)]

// Make sure we don't ICE when suggesting a return type
// for an async fn that has late-bound vars...

async fn ice(_: &i32) {
    true
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

