struct Foo;
struct Bar;

impl From<&Foo> for Bar {
    fn from(foo: &Foo) -> Bar {
        Bar
    }
}

fn main() {
    let foo = Foo;
    let b: Bar = foo.into(); // { dg-error ".E0277." "" { target *-*-* } }
}

