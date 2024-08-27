#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait {
    fn method() {}
}

fn test<T: Trait<method(..): Send>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

