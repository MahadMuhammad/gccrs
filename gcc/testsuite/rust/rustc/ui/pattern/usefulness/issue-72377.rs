#[derive(PartialEq, Eq)]
enum X { A, B, C, }

fn main() {
    let x = X::A;
    let y = Some(X::A);

    match (x, y) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-error ".E0004." "" { target *-*-* } .-2 }
        (_, None) => false,
        (v, Some(w)) if v == w => true,
        (X::B, Some(X::C)) => false,
        (X::B, Some(X::A)) => false,
        (X::A, Some(X::C)) | (X::C, Some(X::A)) => false,
    };
}

