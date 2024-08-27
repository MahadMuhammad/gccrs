//@ revisions: old new
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2021" }
//@[new] run-rustfix
#![deny(bare_trait_objects)]
fn ord_prefer_dot(s: String) -> Ord {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    (s.starts_with("."), s)
}
fn main() {
    let _ = ord_prefer_dot(String::new());
}

