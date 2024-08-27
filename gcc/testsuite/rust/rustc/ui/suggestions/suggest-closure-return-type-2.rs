fn unbound_drop(_: impl Sized) {}

fn main() {
    unbound_drop(|| { [] })
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { help ".E0282." "" { target *-*-* } .-2 }
}

