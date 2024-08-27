fn main() {
    while let 1 = 1 {
        vec![].last_mut().unwrap() = 3_u8;
// { dg-error ".E0070." "" { target *-*-* } .-1 }
    }
}

