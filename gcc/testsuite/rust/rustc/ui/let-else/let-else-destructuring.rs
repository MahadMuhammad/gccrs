#[derive(Debug)]
enum Foo {
    Done,
    Nested(Option<&'static Foo>),
}

fn walk(mut value: &Foo) {
    loop {
        println!("{:?}", value);
        &Foo::Nested(Some(value)) = value else { break }; // { dg-error ".E0070." "" { target *-*-* } }
// { dg-error ".E0070." "" { target *-*-* } .-1 }
    }
}

fn main() {
    walk(&Foo::Done);
}

