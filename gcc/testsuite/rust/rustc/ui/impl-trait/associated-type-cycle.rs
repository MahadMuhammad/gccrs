trait Foo {
    type Bar;
    fn foo(self) -> Self::Bar;
}

impl Foo for Box<dyn Foo> {
// { dg-error ".E0191." "" { target *-*-* } .-1 }
    type Bar = <Self as Foo>::Bar;
    fn foo(self) -> <Self as Foo>::Bar {
        (*self).foo()
    }
}

fn main() {}

