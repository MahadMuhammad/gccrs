trait Baz {
    type Quaks;
}
impl Baz for u8 {
    type Quaks = [u16; 3];
}

trait Bar {}
impl Bar for [u16; 4] {}
impl Bar for [[u16; 3]; 3] {}

trait Foo
where
    [<u8 as Baz>::Quaks; 2]: Bar, // { dg-error ".E0277." "" { target *-*-* } }
    <u8 as Baz>::Quaks: Bar,  // { dg-error ".E0277." "" { target *-*-* } }
{
}

struct FooImpl;

impl Foo for FooImpl {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn f(_: impl Foo) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn main() {
    f(FooImpl)
}

