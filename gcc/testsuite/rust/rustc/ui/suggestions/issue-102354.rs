trait Trait {
    fn func() {}
}

impl Trait for i32 {}

fn main() {
    let x: i32 = 123;
    x.func(); // { dg-error ".E0599." "" { target *-*-* } }
}

