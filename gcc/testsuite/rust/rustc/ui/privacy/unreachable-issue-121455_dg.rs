fn test(s: &Self::Id) {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    match &s[0..3] {}
}

fn main() {}

