fn as_chunks<const N: usize>() -> [u8; N] {
    loop {
        break;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

fn main() {}

