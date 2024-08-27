union U { // { dg-error ".E0072." "" { target *-*-* } }
    a: u8,
    b: std::mem::ManuallyDrop<U>,
}

fn main() {}

