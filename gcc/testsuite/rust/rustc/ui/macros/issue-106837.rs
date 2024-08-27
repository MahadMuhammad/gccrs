fn main() {
    concat!(-42);
    concat!(-3.14);

    concat!(-"hello");
// { dg-error "" "" { target *-*-* } .-1 }

    concat!(--1);
// { dg-error "" "" { target *-*-* } .-1 }
}

