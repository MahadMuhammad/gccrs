struct T1 { // { dg-error ".E0072." "" { target *-*-* } }
    foo: isize,
    foolish: T1,
}

struct T2 { // { dg-error ".E0072." "" { target *-*-* } }
    inner: Option<T2>,
}

type OptionT3 = Option<T3>;

struct T3 { // { dg-error ".E0072." "" { target *-*-* } }
    inner: OptionT3,
}

struct T4(Option<T4>); // { dg-error ".E0072." "" { target *-*-* } }

enum T5 { // { dg-error ".E0072." "" { target *-*-* } }
    Variant(Option<T5>),
}

enum T6 { // { dg-error ".E0072." "" { target *-*-* } }
    Variant{ field: Option<T6> },
}

struct T7 { // { dg-error ".E0072." "" { target *-*-* } }
    foo: std::cell::Cell<Option<T7>>,
}

fn main() { }

