fn main() {
    type Predicate = fn<'a>(&'a str) -> bool;
// { dg-error "" "" { target *-*-* } .-1 }

    type Identity = fn<T>(T) -> T;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }

    let _: fn<const N: usize, 'e, Q, 'f>();
// { dg-error "" "" { target *-*-* } .-1 }

    let _: for<'outer> fn<'inner>();
// { dg-error "" "" { target *-*-* } .-1 }

    let _: for<> fn<'r>();
// { dg-error "" "" { target *-*-* } .-1 }

    type Hmm = fn<>();
// { dg-error "" "" { target *-*-* } .-1 }

    let _: extern fn<'a: 'static>();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    let _: for<'any> extern "C" fn<'u>();
// { dg-error "" "" { target *-*-* } .-1 }

    type QuiteBroken = fn<const>();
// { dg-error "" "" { target *-*-* } .-1 }
}

