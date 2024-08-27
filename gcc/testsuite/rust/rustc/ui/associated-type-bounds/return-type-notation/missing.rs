// { dg-additional-options "-frust-edition= 2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait {
    async fn method() {}
}

fn bar<T: Trait<methid(..): Send>>() {}
// { dg-error ".E0220." "" { target *-*-* } .-1 }

fn main() {}

