trait AsPtr {
    type Ptr;
}

impl AsPtr for () {
    type Ptr = *const void;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

#[derive(Copy, Clone)]
struct Foo {
    p: <() as AsPtr>::Ptr,
    // Do not report a "`Copy` cannot be implemented" here.
}

fn main() {}

