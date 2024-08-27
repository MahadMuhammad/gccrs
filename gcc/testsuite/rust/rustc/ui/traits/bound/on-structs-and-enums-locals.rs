trait Trait {
    fn dummy(&self) { }
}

struct Foo<T:Trait> {
    x: T,
}

fn main() {
    let foo = Foo {
        x: 3
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    };

    let baz: Foo<usize> = loop { };
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

