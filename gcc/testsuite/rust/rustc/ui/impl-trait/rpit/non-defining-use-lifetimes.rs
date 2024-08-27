// issue: #111935

#![allow(unconditional_recursion)]

// Lt indirection is necessary to make the lifetime of the function late-bound,
// in order to bypass some other bugs.
type Lt<'lt> = Option<*mut &'lt ()>;

mod statik {
    use super::*;
    // invalid defining use: Opaque<'static> := ()
    fn foo<'a>(_: Lt<'a>) -> impl Sized + 'a {
        let _: () = foo(Lt::<'static>::None);
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

mod infer {
    use super::*;
    // invalid defining use: Opaque<'_> := ()
    fn foo<'a>(_: Lt<'a>) -> impl Sized + 'a {
        let _: () = foo(Lt::<'_>::None);
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

mod equal {
    use super::*;
    // invalid defining use: Opaque<'a, 'a> := ()
    // because of the use of equal lifetimes in args
    fn foo<'a, 'b>(_: Lt<'a>, _: Lt<'b>) -> impl Sized + 'a + 'b {
        let _: () = foo(Lt::<'a>::None, Lt::<'a>::None);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

