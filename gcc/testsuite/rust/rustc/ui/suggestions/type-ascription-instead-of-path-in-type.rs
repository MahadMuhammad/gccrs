enum A {
    B,
}

fn main() {
    let _: Vec<A:B> = A::B;
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }
// { help ".E0229." "" { target *-*-* } .-4 }
// { dg-error ".E0229." "" { target *-*-* } .-5 }
}

