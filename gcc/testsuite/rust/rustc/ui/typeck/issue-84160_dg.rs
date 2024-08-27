fn mismatched_types_with_reference(x: &u32) -> &u32 {
    if false {
        return x;
    }
    return "test";
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

