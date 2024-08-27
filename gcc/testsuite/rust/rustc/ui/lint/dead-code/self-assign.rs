// Test that dead code warnings are issued for superfluous assignments of
// fields or variables to themselves (issue #75356).

//@ ignore-test FIXME(81658, 83171)

//@ check-pass
#![allow(unused_assignments)]
#![warn(dead_code)]

fn main() {
    let mut x = 0;
    x = x;
// { dg-warning "" "" { target *-*-* } .-1 }

    x = (x);
// { dg-warning "" "" { target *-*-* } .-1 }

    x = {x};
    // block expressions don't count as self-assignments


    struct S<'a> { f: &'a str }
    let mut s = S { f: "abc" };
    s = s;
// { dg-warning "" "" { target *-*-* } .-1 }

    s.f = s.f;
// { dg-warning "" "" { target *-*-* } .-1 }


    struct N0 { x: Box<i32> }
    struct N1 { n: N0 }
    struct N2(N1);
    struct N3 { n: N2 };
    let mut n3 = N3 { n: N2(N1 { n: N0 { x: Box::new(42) } }) };
    n3.n.0.n.x = n3.n.0.n.x;
// { dg-warning "" "" { target *-*-* } .-1 }

    let mut t = (1, ((2, 3, (4, 5)),));
    t.1.0.2.1 = t.1.0.2.1;
// { dg-warning "" "" { target *-*-* } .-1 }


    let mut y = 0;
    macro_rules! assign_to_y {
        ($cur:expr) => {{
            y = $cur;
        }};
    }
    assign_to_y!(y);
    // self-assignments in macro expansions are not reported either
}

