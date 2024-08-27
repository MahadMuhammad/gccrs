fn main() {
    let x = |a: u8, b: (usize, u32), c: fn() -> char| -> String { "I love beans.".to_string() };
// { dg-note "" "" { target *-*-* } .-1 }

    let x: fn(i32) = x;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
// { dg-note ".E0308." "" { target *-*-* } .-5 }
}

