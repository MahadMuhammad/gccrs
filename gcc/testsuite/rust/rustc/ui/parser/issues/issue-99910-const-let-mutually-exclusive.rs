//@ run-rustfix

fn main() {
    const let _FOO: i32 = 123;
// { dg-error "" "" { target *-*-* } .-1 }
    let const _BAR: i32 = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}

