fn main() {}

fn foo() {
    #[derive(Copy, Clone)]
    struct Foo(NonExistent);
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }

    type U = impl Copy;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    let foo: U = Foo(());
    let Foo(()) = foo;
}

