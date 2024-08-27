#[derive(PartialEq)] struct Comparable;
#[derive(PartialEq, PartialOrd)] struct Nope(Comparable);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}

