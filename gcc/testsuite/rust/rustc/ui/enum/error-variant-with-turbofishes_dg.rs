enum Struct<const N: usize> { Variant { x: [(); N] } }

fn test() {
    let x = Struct::<0>::Variant;
// { dg-error ".E0533." "" { target *-*-* } .-1 }
}

fn main() {}

