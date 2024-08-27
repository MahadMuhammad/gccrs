macro_rules! m {
    ($s:stmt) => {}
}

m! { mut x }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
m! { auto x }
// { dg-error "" "" { target *-*-* } .-1 }
m! { var x }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

