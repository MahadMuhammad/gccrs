fn main() {}

struct S {
    fn: u8 // { dg-error "" "" { target *-*-* } }
}

