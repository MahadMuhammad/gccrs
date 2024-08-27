enum Option<T> {
    Some(T),
    None,
}

pub fn foo() -> Option<u8> {
    Some(42_u8)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

