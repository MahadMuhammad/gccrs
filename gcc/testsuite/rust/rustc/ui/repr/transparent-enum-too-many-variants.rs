use std::mem::size_of;

#[repr(transparent)]
enum Foo { // { dg-error ".E0731." "" { target *-*-* } }
    A(u8), B(u8),
}

fn main() {
    println!("Foo: {}", size_of::<Foo>());
}

