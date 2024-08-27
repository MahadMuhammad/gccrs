#[allow(unused)]
fn foo() { // { help "" "" { target *-*-* } }
    vec!['a'].iter().map(|c| c)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { help ".E0308." "" { target *-*-* } .-4 }
}

fn main() {}

