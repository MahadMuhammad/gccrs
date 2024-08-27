fn test() -> (i32,) {
    return;
// { dg-error ".E0069." "" { target *-*-* } .-1 }
}

fn main() {}

