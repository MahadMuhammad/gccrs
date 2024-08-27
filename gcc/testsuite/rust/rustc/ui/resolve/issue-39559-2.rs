trait Dim {
    fn dim() -> usize;
}

enum Dim3 {}

impl Dim for Dim3 {
    fn dim() -> usize {
        3
    }
}

fn main() {
    let array: [usize; Dim3::dim()]
// { dg-error ".E0015." "" { target *-*-* } .-1 }
        = [0; Dim3::dim()];
// { dg-error ".E0015." "" { target *-*-* } .-1 }
}

