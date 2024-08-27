trait Foo {
    const ASSOC: [u8];
}

fn bar<T: Foo>() {
    let a = [T::ASSOC; 2];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

