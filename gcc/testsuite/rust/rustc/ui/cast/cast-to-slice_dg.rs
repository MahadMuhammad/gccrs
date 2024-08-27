fn main() {
    "example".as_bytes() as [char];
// { dg-error ".E0620." "" { target *-*-* } .-1 }

    let arr: &[u8] = &[0, 2, 3];
    arr as [char];
// { dg-error ".E0620." "" { target *-*-* } .-1 }
}

