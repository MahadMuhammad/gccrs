//@ incremental

struct Wrapper<T>(T);

struct Local<T, U>(T, U);

impl<T> Local { // { dg-error ".E0107." "" { target *-*-* } }
    type AssocType3 = T; // { dg-error ".E0658." "" { target *-*-* } }

    const WRAPPED_ASSOC_3: Wrapper<Self::AssocType3> = Wrapper();
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}
// { dg-error ".E0601." "" { target *-*-* } .-1 }

