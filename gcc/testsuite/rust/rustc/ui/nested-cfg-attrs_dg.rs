#[cfg_attr(all(), cfg_attr(all(), cfg(FALSE)))]
fn f() {}

fn main() { f() } // { dg-error ".E0425." "" { target *-*-* } }

