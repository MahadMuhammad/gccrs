
fn main() {
    |x: usize| [0; x];  // { dg-error ".E0435." "" { target *-*-* } }
    // (note the newline before "fn")
}
// ignore-tidy-leading-newlines

