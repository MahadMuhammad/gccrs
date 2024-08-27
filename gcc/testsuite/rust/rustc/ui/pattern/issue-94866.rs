macro_rules! m {
    () => {
        {}
    };
}

enum Enum { A, B }

fn main() {
    match Enum::A {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
    Enum::A => m!()
    }
}

