#[unstable()] // { dg-error ".E0734." "" { target *-*-* } }
#[stable()] // { dg-error ".E0734." "" { target *-*-* } }
fn main() {}

