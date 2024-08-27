// Test that we do not suggest to add type annotations for unnamable types.

#![crate_type="lib"]
#![feature(coroutines, stmt_expr_attributes)]

const A = 5;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

static B: _ = "abc";
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }


// FIXME: this should also suggest a function pointer, as the closure is non-capturing
const C: _ = || 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { dg-note ".E0121." "" { target *-*-* } .-3 }

struct S<T> { t: T }
const D = S { t: { let i = 0; move || -> i32 { i } } };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }


fn foo() -> i32 { 42 }
const E = foo;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
const F = S { t: foo };
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }


const G = #[coroutine] || -> i32 { yield 0; return 1; };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

