#![crate_name = "time"]

fn main() {
    let items = Box::new(vec![]); // { dg-error ".E0282." "" { target *-*-* } }
// { dg-note ".E0282." "" { target *-*-* } .-1 }
// { dg-note ".E0282." "" { target *-*-* } .-2 }
    items.into();
}

