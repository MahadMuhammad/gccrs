//@ run-rustfix

fn add_ten<N>(n: N) -> N {
    n + 10
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

fn main() { add_ten(0); }

