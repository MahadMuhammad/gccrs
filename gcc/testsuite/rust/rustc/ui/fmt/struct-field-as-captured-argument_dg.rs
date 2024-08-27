//@ run-rustfix

#[derive(Debug)]
struct Foo {
    field: usize,
}

fn main() {
    let foo = Foo { field: 0 };
    let bar = 3;
    let _ = format!("{foo.field}"); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field} {} {bar}", "aa"); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field} {} {1} {bar}", "aa", "bb"); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field} {} {baz}", "aa", baz = 3); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field:?} {} {baz}", "aa", baz = 3); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field:#?} {} {baz}", "aa", baz = 3); // { dg-error "" "" { target *-*-* } }
    let _ = format!("{foo.field:.3} {} {baz}", "aa", baz = 3); // { dg-error "" "" { target *-*-* } }
}

