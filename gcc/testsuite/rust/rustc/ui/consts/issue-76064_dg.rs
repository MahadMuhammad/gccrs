struct Bug([u8; panic!("panic")]); // { dg-error ".E0080." "" { target *-*-* } }

fn main() {}

