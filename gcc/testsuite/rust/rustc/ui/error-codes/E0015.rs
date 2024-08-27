fn create_some() -> Option<u8> {
    Some(1)
}

const FOO: Option<u8> = create_some();
// { dg-error ".E0015." "" { target *-*-* } .-1 }

fn main() {}

