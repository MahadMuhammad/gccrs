fn main() {
    let vec = Vec::new();
    Vec::contains(&vec, &0);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

