enum Foo {
    Bar { a: u8, b: i8, c: u8 },
    Baz { a: f32 },
    None,
}

fn main() {
    let foo = Foo::None;
    match foo {
        Foo::Bar { a, aa: 1, c } => (),
// { dg-error ".E0027." "" { target *-*-* } .-1 }
// { dg-error ".E0027." "" { target *-*-* } .-2 }
        Foo::Baz { bb: 1.0 } => (),
// { dg-error ".E0027." "" { target *-*-* } .-1 }
// { dg-error ".E0027." "" { target *-*-* } .-2 }
        _ => (),
    }

    match foo {
        Foo::Bar { a, aa: "", c } => (),
// { dg-error ".E0027." "" { target *-*-* } .-1 }
// { dg-error ".E0027." "" { target *-*-* } .-2 }
        Foo::Baz { bb: "" } => (),
// { dg-error ".E0027." "" { target *-*-* } .-1 }
// { dg-error ".E0027." "" { target *-*-* } .-2 }
        _ => (),
    }
}

