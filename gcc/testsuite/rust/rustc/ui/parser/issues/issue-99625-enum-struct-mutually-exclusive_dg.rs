//@ run-rustfix

pub enum struct Range {
// { dg-error "" "" { target *-*-* } .-1 }
    Valid {
        begin: u32,
        len: u32,
    },
    Out,
}

fn main() {
}

