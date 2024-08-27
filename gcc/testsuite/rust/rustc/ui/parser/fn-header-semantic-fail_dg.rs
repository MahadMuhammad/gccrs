// Ensures that all `fn` forms can have all the function qualifiers syntactically.

// { dg-additional-options "-frust-edition=2018" }

#![feature(const_extern_fn)]

fn main() {
    async fn ff1() {} // OK.
    unsafe fn ff2() {} // OK.
    const fn ff3() {} // OK.
    extern "C" fn ff4() {} // OK.
    const async unsafe extern "C" fn ff5() {}
// { dg-error "" "" { target *-*-* } .-1 }

    trait X {
        async fn ft1(); // OK.
        unsafe fn ft2(); // OK.
        const fn ft3(); // { dg-error ".E0379." "" { target *-*-* } }
        extern "C" fn ft4(); // OK.
        const async unsafe extern "C" fn ft5();
// { dg-error ".E0379." "" { target *-*-* } .-1 }
// { dg-error ".E0379." "" { target *-*-* } .-2 }
    }

    struct Y;
    impl X for Y {
        async fn ft1() {} // OK.
        unsafe fn ft2() {} // OK.
        const fn ft3() {} // { dg-error ".E0379." "" { target *-*-* } }
        extern "C" fn ft4() {}
        const async unsafe extern "C" fn ft5() {}
// { dg-error ".E0379." "" { target *-*-* } .-1 }
// { dg-error ".E0379." "" { target *-*-* } .-2 }
    }

    impl Y {
        async fn fi1() {} // OK.
        unsafe fn fi2() {} // OK.
        const fn fi3() {} // OK.
        extern "C" fn fi4() {} // OK.
        const async unsafe extern "C" fn fi5() {}
// { dg-error "" "" { target *-*-* } .-1 }
    }

    extern "C" {
        async fn fe1(); // { dg-error "" "" { target *-*-* } }
        unsafe fn fe2(); // { dg-error "" "" { target *-*-* } }
        const fn fe3(); // { dg-error "" "" { target *-*-* } }
        extern "C" fn fe4(); // { dg-error "" "" { target *-*-* } }
        const async unsafe extern "C" fn fe5();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
    }
}

