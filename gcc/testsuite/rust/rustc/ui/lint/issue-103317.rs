//@ check-pass
//@ run-rustfix

#[warn(unreachable_pub)]
mod inner {
    #[allow(unused)]
    pub enum T {
// { dg-warning "" "" { target *-*-* } .-1 }
        A(u8),
        X { a: f32, b: () },
    }
}

fn main() {}

