//@ run-rustfix
fn main() {
    let value = [7u8];
    while Some(0) = value.get(0) {} // { dg-error ".E0070." "" { target *-*-* } }
}

