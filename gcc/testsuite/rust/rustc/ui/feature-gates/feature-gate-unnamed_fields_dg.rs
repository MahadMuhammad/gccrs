#[repr(C)]
struct Foo {
    foo: u8,
    _: union { // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        bar: u8,
        baz: u16
    }
}

#[repr(C)]
union Bar {
    foobar: u8,
    _: struct { // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        foobaz: u8,
        barbaz: u16
    }
}

#[repr(C)]
struct S;

#[repr(C)]
struct Baz {
    _: S // { dg-error ".E0658." "" { target *-*-* } }
}

fn main(){}

