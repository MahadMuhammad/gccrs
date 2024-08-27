#[derive(PartialEq)]
enum E {
    A,
}

const E_SL: &[E] = &[E::A];

fn main() {
    match &[][..] {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        E_SL => {}
    }
}

