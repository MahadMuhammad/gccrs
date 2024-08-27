fn main() {
    let a = concat_bytes!(b'A', b"BC"); // { dg-error ".E0658." "" { target *-*-* } }
    assert_eq!(a, &[65, 66, 67]);
}

