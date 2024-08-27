//@ run-rustfix
// Check that the HELP suggestion is `l(vec![])` instead of `l($crate::vec::Vec::new())`
fn l(_a: Vec<u8>) {}

fn main() {
    l(vec![], vec![])
// { dg-error ".E0061." "" { target *-*-* } .-1 }
// { help ".E0061." "" { target *-*-* } .-2 }
}

