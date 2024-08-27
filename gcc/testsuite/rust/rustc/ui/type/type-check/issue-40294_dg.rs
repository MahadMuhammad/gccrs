trait Foo: Sized {
    fn foo(self);
}

fn foo<'a,'b,T>(x: &'a T, y: &'b T)
    where &'a T : Foo, // { dg-error ".E0283." "" { target *-*-* } }
          &'b T : Foo
{
    x.foo(); // { dg-error ".E0283." "" { target *-*-* } }
    y.foo();
}

fn main() { }

