pub trait Trait<A> {
    const ONE<T>: i32;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

    const TWO: ()
    where
        A: Copy;
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

const CONST<T>: i32 = 0;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

const EMPTY<>: i32 = 0;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

const TRUE: () = ()
where
    String: Clone;
// { dg-error ".E0658." "" { target *-*-* } .-2 }

// Ensure that we flag generic const items inside macro calls as well:

macro_rules! discard {
    ($item:item) => {}
}

discard! { const FREE<T>: () = (); }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

discard! { impl () { const ASSOC<const N: ()>: () = (); } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

discard! { impl () { const ASSOC: i32 = 0 where String: Copy; } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

