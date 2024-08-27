fn missing() -> impl Sized + use<T> {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn missing_self() -> impl Sized + use<Self> {}
// { dg-error ".E0411." "" { target *-*-* } .-1 }

struct MyType;
impl MyType {
    fn self_is_not_param() -> impl Sized + use<Self> {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn hello() -> impl Sized + use<hello> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

