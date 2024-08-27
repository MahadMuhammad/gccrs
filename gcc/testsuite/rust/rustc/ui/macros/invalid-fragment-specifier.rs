macro_rules! test {
    ($wrong:id) => {};
// { dg-error "" "" { target *-*-* } .-1 }

// guard against breaking raw identifier diagnostic
macro_rules! test_raw_identifer {
    ($wrong:r#if) => {};
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

