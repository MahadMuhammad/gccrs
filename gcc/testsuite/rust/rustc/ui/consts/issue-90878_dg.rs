 fn main() {
    |x: usize| [0; x];  // { dg-error ".E0435." "" { target *-*-* } }
    // (note the space before "fn")
}

