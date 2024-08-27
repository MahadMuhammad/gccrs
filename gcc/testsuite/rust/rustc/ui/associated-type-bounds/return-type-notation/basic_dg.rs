//@ revisions: with without
// { dg-additional-options "-frust-edition= 2021" }
//@ [with] check-pass

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Foo {
    async fn method() -> Result<(), ()>;
}

async fn foo<T: Foo>() -> Result<(), ()> {
    T::method().await?;
    Ok(())
}

fn is_send(_: impl Send) {}

fn test<
    #[cfg(with)] T: Foo<method(..): Send>,
    #[cfg(without)] T: Foo,
>() {
    is_send(foo::<T>());
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

