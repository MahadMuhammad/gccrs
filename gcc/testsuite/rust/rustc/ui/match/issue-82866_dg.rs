fn main() {
    match x {
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        Some::<v>(v) => (),
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    }
}

