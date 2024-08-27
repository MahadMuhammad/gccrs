struct Wrapper<G: Send>(G);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

