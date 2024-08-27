fn main() {
    let x = &(&0u8 as u8); // { dg-error ".E0606." "" { target *-*-* } }
    x as u8; // { dg-error ".E0606." "" { target *-*-* } }
}

