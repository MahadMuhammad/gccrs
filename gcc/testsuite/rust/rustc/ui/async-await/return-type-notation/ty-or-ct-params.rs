// { dg-additional-options "-frust-edition= 2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Foo {
    async fn bar<T>() {}

    async fn baz<const N: usize>() {}
}

fn test<T>()
where
    T: Foo<bar(..): Send, baz(..): Send>,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
{
}

fn main() {}

