//@ run-rustfix

const _A: = 123;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    const _B: = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}

