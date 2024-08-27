// Checks whether shadowing a const parameter leads to an ICE (#85348).

impl<const N: usize> ArrayWindowsExample {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    fn next() {
        let mut N;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { dg-error ".E0282." "" { target *-*-* } .-2 }
    }
}

fn main() {}

