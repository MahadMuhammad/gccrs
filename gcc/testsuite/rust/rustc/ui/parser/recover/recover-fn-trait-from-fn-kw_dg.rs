fn foo(_: impl fn() -> i32) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn foo2<T: fn(i32)>(_: T) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    foo(|| ());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    foo2(|_: ()| {});
// { dg-error ".E0631." "" { target *-*-* } .-1 }
}

