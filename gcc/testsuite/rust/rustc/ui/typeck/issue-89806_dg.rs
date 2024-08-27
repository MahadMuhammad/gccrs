fn main() {
    0u8.as_ref(); // { dg-error ".E0599." "" { target *-*-* } }
}

