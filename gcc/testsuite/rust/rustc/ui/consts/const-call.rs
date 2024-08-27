fn f(x: usize) -> usize {
    x
}

fn main() {
    let _ = [0; f(2)];
// { dg-error ".E0015." "" { target *-*-* } .-1 }
}

