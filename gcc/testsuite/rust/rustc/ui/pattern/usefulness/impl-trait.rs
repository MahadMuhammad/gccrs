#![feature(never_type)]
#![feature(type_alias_impl_trait)]
#![feature(non_exhaustive_omitted_patterns_lint)]
#![deny(unreachable_patterns)]
// Test that the lint traversal handles opaques correctly
#![deny(non_exhaustive_omitted_patterns)]

fn main() {}

#[derive(Copy, Clone)]
enum Void {}

fn return_never_rpit(x: Void) -> impl Copy {
    if false {
        match return_never_rpit(x) {
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    x
}
fn friend_of_return_never_rpit(x: Void) {
    match return_never_rpit(x) {}
// { dg-error ".E0004." "" { target *-*-* } .-1 }
}

type T = impl Copy;
fn return_never_tait(x: Void) -> T {
    if false {
        match return_never_tait(x) {
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    x
}
fn friend_of_return_never_tait(x: Void) {
    match return_never_tait(x) {}
// { dg-error ".E0004." "" { target *-*-* } .-1 }
}

fn option_never(x: Void) -> Option<impl Copy> {
    if false {
        match option_never(x) {
            None => {}
            Some(_) => {} // { dg-error "" "" { target *-*-* } }
        }
        match option_never(x) {
            None => {}
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    Some(x)
}

fn option_never2(x: Void) -> impl Copy {
    if false {
        match option_never2(x) {
            None => {}
            Some(_) => {} // { dg-error "" "" { target *-*-* } }
        }
        match option_never2(x) {
            None => {}
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
        match option_never2(x) {
            None => {}
        }
    }
    Some(x)
}

fn inner_never(x: Void) {
    type T = impl Copy;
    let y: T = x;
    match y {
        _ => {} // { dg-error "" "" { target *-*-* } }
    }
}

// This one caused ICE https://github.com/rust-lang/rust/issues/117100.
fn inner_tuple() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    match foo {
        _ => {}
        Some((a, b)) => {} // { dg-error "" "" { target *-*-* } }
    }
}

type U = impl Copy;
fn unify_never(x: Void, u: U) -> U {
    if false {
        match u {
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    x
}

type V = impl Copy;
fn infer_in_match(x: Option<V>) {
    match x {
        None => {}
        Some((a, b)) => {}
        Some((mut x, mut y)) => {
// { dg-error "" "" { target *-*-* } .-1 }
            x = 42;
            y = "foo";
        }
    }
}

type W = impl Copy;
#[derive(Copy, Clone)]
struct Rec<'a> {
    n: u32,
    w: Option<&'a W>,
}
fn recursive_opaque() -> W {
    if false {
        match recursive_opaque() {
            // Check for the ol' ICE when the type is recursively opaque.
            _ => {}
            Rec { n: 0, w: Some(Rec { n: 0, w: _ }) } => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    let w: Option<&'static W> = None;
    Rec { n: 0, w }
}

type X = impl Copy;
struct SecretelyVoid(X);
fn nested_empty_opaque(x: Void) -> X {
    if false {
        let opaque_void = nested_empty_opaque(x);
        let secretely_void = SecretelyVoid(opaque_void);
        match secretely_void {
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    x
}

type Y = (impl Copy, impl Copy);
struct SecretelyDoubleVoid(Y);
fn super_nested_empty_opaque(x: Void) -> Y {
    if false {
        let opaque_void = super_nested_empty_opaque(x);
        let secretely_void = SecretelyDoubleVoid(opaque_void);
        match secretely_void {
            _ => {} // { dg-error "" "" { target *-*-* } }
        }
    }
    (x, x)
}

