// { dg-additional-options "-frust-edition=2021" }


struct Wrapper<T>(T);

trait Foo {
    fn bar() -> Wrapper<Missing<impl Sized>>;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl Foo for () {
    fn bar() -> Wrapper<i32> {
        Wrapper(0)
    }
}

fn main() {}

