// Regression test for issue #45045

enum Xyz {
    A,
    B,
}

fn main() {
    let mut e = Xyz::A;
    let f = &mut e;
    let g = f;
    match e {
// { dg-error ".E0503." "" { target *-*-* } .-1 }
        Xyz::A => println!("a"),
        Xyz::B => println!("b"),
    };
    *g = Xyz::B;
}

