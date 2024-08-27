fn test<const N: usize, const M: usize>() -> [u8; M] {
    [0; N] // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {}

