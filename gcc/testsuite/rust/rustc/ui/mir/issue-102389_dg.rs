enum Enum { A, B, C }

fn func(inbounds: &Enum, array: &[i16; 3]) -> i16 {
    array[*inbounds as usize]
// { dg-error ".E0507." "" { target *-*-* } .-1 }
}

fn main() {}

