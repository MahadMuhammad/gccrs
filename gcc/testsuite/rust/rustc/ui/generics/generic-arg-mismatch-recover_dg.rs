struct Foo<'a, T: 'a>(&'a T);

struct Bar<'a>(&'a ());

fn main() {
    Foo::<'static, 'static, ()>(&0);
// { dg-error ".E0107." "" { target *-*-* } .-1 }

    Bar::<'static, 'static, ()>(&());
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
}

