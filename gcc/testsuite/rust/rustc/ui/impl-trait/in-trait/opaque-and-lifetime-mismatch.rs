struct Wrapper<'rom>(&'rom ());

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
}

impl Foo for () {
    fn bar() -> i32 {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
// { dg-error ".E0053." "" { target *-*-* } .-2 }
        0
    }
}

trait Bar {
    fn foo() -> Wrapper<impl Sized>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
}

impl Bar for () {
    fn foo() -> Wrapper<impl Sized> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
        Wrapper(&())
    }
}

fn main() {}

