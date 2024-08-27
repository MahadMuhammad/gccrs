struct FooStruct {
    nested: &'static Bar<dyn std::fmt::Debug>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct FooTuple(&'static Bar<dyn std::fmt::Debug>);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

enum FooEnum1 {
    Struct { nested: &'static Bar<dyn std::fmt::Debug> },
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

enum FooEnum2 {
    Tuple(&'static Bar<dyn std::fmt::Debug>),
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct Bar<T>(T);

fn main() {
    // Ensure there's an error at the construction site, for error tainting purposes.

    FooStruct { nested: &Bar(4) };
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    FooTuple(&Bar(4));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    FooEnum1::Struct { nested: &Bar(4) };
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    FooEnum2::Tuple(&Bar(4));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

