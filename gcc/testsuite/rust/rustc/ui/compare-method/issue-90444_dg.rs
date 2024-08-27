pub struct A;
impl From<fn((), (), &())> for A {
    fn from(_: fn((), (), &mut ())) -> Self {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        loop {}
    }
}

pub struct B;
impl From<fn((), (), u32)> for B {
    fn from(_: fn((), (), u64)) -> Self {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        loop {}
    }
}

fn main() {}

