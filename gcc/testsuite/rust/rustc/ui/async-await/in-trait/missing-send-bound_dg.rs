// { dg-additional-options "-frust-edition=2021" }


trait Foo {
    async fn bar();
}

async fn test<T: Foo>() {
    T::bar().await;
}

fn test2<T: Foo>() {
    assert_is_send(test::<T>());
// { dg-error "" "" { target *-*-* } .-1 }
}

fn assert_is_send(_: impl Send) {}

fn main() {}

