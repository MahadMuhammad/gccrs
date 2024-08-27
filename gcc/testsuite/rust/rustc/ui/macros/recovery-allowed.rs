macro_rules! please_recover {
    ($a:expr) => {};
}

please_recover! { not 1 }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

