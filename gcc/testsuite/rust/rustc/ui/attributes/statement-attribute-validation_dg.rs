// test for #117058 - check that attributes are validated on various kinds of statements.

struct A;

fn func() {}

fn main() {
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    if true {
    } else {
    }
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    (1);
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    match 1 {
        _ => {}
    }
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    while false {}
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    {}
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    A {};
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    func();
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    A;
    #[allow(two-words)]
// { dg-error "" "" { target *-*-* } .-1 }
    loop {}
}

