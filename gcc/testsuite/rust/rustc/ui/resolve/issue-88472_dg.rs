// Regression test for #88472, where a suggestion was issued to
// import an inaccessible struct.

#![warn(unused_imports)]
// { dg-note "" "" { target *-*-* } .-1 }

mod a {
    struct Foo;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

mod b {
    use crate::a::*;
// { dg-warning "" "" { target *-*-* } .-1 }
    type Bar = Foo;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-note ".E0412." "" { target *-*-* } .-2 }
}

mod c {
    enum Eee {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

    mod d {
        enum Eee {}
// { dg-note "" "" { target *-*-* } .-1 }
    }
}

mod e {
    type Baz = Eee;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-note ".E0412." "" { target *-*-* } .-2 }
}

fn main() {}

