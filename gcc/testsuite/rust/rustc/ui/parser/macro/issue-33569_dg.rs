macro_rules! foo {
    { $+ } => { // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
        $(x)(y) // { dg-error "" "" { target *-*-* } }
    }
}

foo!();

fn main() {}

