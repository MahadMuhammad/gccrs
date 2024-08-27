trait Bar {
    type Baz;
}

struct Foo<T> where T: Bar, <T as Bar>::Baz: String { // { dg-error ".E0404." "" { target *-*-* } }
    t: T,
}

struct Qux<'a, T> where T: Bar, <&'a T as Bar>::Baz: String { // { dg-error ".E0404." "" { target *-*-* } }
    t: &'a T,
}

fn foo<T: Bar>(_: T) where <T as Bar>::Baz: String { // { dg-error ".E0404." "" { target *-*-* } }
}

fn qux<'a, T: Bar>(_: &'a T) where <&'a T as Bar>::Baz: String { // { dg-error ".E0404." "" { target *-*-* } }
}

fn issue_95327() where <u8 as Unresolved>::Assoc: String {}
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { dg-error ".E0404." "" { target *-*-* } .-2 }

fn main() {}

