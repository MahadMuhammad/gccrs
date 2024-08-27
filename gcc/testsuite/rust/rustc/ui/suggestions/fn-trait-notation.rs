//@ run-rustfix
fn e0658<F, G, H>(f: F, g: G, h: H) -> i32
where
    F: Fn<i32, Output = i32>, // { dg-error ".E0059." "" { target *-*-* } }
// { dg-error ".E0059." "" { target *-*-* } .-1 }
    G: Fn<(i32, i32, ), Output = (i32, i32)>, // { dg-error ".E0658." "" { target *-*-* } }
    H: Fn<(i32,), Output = i32>, // { dg-error ".E0658." "" { target *-*-* } }
{
    f(3);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    g(3, 4);
    h(3)
}

fn main() {
    e0658( // { dg-error ".E0308." "" { target *-*-* } }
        |a| a,
        |a, b| (b, a),
        |a| a,
    );
}

