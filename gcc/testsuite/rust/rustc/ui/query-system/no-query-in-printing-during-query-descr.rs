fn a() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    &a
}

fn main() {}

