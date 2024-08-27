struct Wrapper<T>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        0
    }
}

fn main() {}

