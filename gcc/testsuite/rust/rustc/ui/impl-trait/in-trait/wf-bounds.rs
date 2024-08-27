// issue #101663

use std::fmt::Display;

trait Wf<T> {
    type Output;
}

struct NeedsDisplay<T: Display>(T);

trait Uwu {
    fn nya() -> impl Wf<Vec<[u8]>>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    fn nya2() -> impl Wf<[u8]>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    fn nya3() -> impl Wf<(), Output = impl Wf<Vec<[u8]>>>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    fn nya4<T>() -> impl Wf<NeedsDisplay<T>>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

