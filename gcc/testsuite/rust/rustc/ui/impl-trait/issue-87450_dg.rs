fn bar() -> impl Fn() {
    wrap(wrap(wrap(wrap(foo()))))
}

fn foo() -> impl Fn() {
// { dg-warning ".E0720." "" { target *-*-* } .-1 }
// { dg-error ".E0720." "" { target *-*-* } .-2 }
    wrap(wrap(wrap(wrap(wrap(wrap(wrap(foo())))))))
}

fn wrap(f: impl Fn()) -> impl Fn() {
    move || f()
}

fn main() {
}

