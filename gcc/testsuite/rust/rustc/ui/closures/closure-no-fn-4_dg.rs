fn main() {
    let b = 2;
    let _: fn(usize) -> usize = match true {
        true => |a| a + 1,
        false => |a| a - b,
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

