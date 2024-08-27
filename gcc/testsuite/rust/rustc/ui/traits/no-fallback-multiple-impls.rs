trait Fallback {
    fn foo(&self) {}
}

impl Fallback for i32 {}

impl Fallback for u64 {}

impl Fallback for usize {}

fn main() {
    missing();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    0.foo();
    // But then we shouldn't report an inference ambiguity here...
}

