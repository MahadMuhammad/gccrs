trait Trait {
    type Foo<const N: u8>;
}

impl Trait for () {
    type Foo<const N: u64> = u32;
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

fn main() {}

