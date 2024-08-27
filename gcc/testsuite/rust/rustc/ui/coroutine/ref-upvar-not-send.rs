// For `Send` coroutines, suggest a `T: Sync` requirement for `&T` upvars,
// and suggest a `T: Send` requirement for `&mut T` upvars.

#![feature(coroutines, stmt_expr_attributes)]

fn assert_send<T: Send>(_: T) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

fn main() {
    let x: &*mut () = &std::ptr::null_mut();
    let y: &mut *mut () = &mut std::ptr::null_mut();
    assert_send(#[coroutine] move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        yield;
        let _x = x;
    });
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-2 }
    assert_send(#[coroutine] move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        yield;
        let _y = y;
    });
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-2 }
}

