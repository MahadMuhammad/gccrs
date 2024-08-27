//@ run-rustfix

fn main() {
    mut let _x = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}

