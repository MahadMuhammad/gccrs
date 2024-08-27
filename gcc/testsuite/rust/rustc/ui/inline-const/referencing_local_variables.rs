const fn test_me<T>(a: usize) -> usize {
    const { a }
// { dg-error ".E0435." "" { target *-*-* } .-1 }
}

fn main() {}

