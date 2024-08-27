fn main() {
    #[derive(Debug)]
    enum Foo {
// { help "" "" { target *-*-* } .-1 }
        Bar,
        Qux,
    }

    let vec1 = vec![Foo::Bar, Foo::Qux];
    let vec2 = vec![Foo::Bar, Foo::Qux];
    assert_eq!(vec1, vec2);
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

