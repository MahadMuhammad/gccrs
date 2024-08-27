fn foo<const X: usize>() -> usize {
    0
}

fn main() {
    foo(); // { dg-error ".E0284." "" { target *-*-* } }
}

