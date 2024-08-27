// issue: rust-lang/rust#125081

fn main() {
    let _: &str = 'β;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

