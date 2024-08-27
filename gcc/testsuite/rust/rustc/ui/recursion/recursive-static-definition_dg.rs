pub static FOO: u32 = FOO;
// { dg-error ".E0080." "" { target *-*-* } .-1 }

#[derive(Copy, Clone)]
pub union Foo {
    x: u32,
}

pub static BAR: Foo = BAR;
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {}

