//@ run-rustfix

use std::ops::Index;
struct X;
impl Index<i32> for X {
    type Output = ();

    fn index(&self, _: i32) -> &() {
        &()
    }
}

fn main() {
    let x = vec![1, 2, 3];
    x[-1]; // { dg-error "" "" { target *-*-* } }
    let x = [1, 2, 3];
    x[-1]; // { dg-error "" "" { target *-*-* } }
    let x = &[1, 2, 3];
    x[-1]; // { dg-error "" "" { target *-*-* } }
    let _ = x;
    X[-1];
}

