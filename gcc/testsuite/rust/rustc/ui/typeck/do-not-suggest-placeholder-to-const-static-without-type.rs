trait Foo {
    const A; // { dg-error "" "" { target *-*-* } }
    static B;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

