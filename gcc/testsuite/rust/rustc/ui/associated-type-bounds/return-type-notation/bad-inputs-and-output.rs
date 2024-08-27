// { dg-additional-options "-frust-edition= 2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait {
    async fn method() {}
}

fn foo<T: Trait<method(i32): Send>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn bar<T: Trait<method() -> (): Send>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn baz<T: Trait<method(): Send>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

