enum Enum<'a, T, 'b> {
// { dg-error "" "" { target *-*-* } .-1 }
    A(&'a &'b T)
}

struct Struct<'a, T, 'b> {
// { dg-error "" "" { target *-*-* } .-1 }
    x: &'a &'b T
}

trait Trait<'a, T, 'b> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn foo<'a, T, 'b>(x: &'a T) {}
// { dg-error "" "" { target *-*-* } .-1 }

struct Y<T>(T);
impl<'a, T, 'b> Y<T> {}
// { dg-error "" "" { target *-*-* } .-1 }

mod bar {
    pub struct X<'a, 'b, 'c, T> {
        a: &'a str,
        b: &'b str,
        c: &'c str,
        t: T,
    }
}

fn bar<'a, 'b, 'c, T>(x: bar::X<'a, T, 'b, 'c>) {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }

fn main() {}

