fn main() {
    let abs: i32 = 3i32.checked_abs();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

