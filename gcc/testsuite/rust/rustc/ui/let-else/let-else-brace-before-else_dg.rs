//@ run-rustfix



fn main() {
    let Some(1) = { Some(1) } else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };
    let 2 = 1 + match 1 { n => n } else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };
    let Some(1) = unsafe { unsafe_fn() } else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };
}

unsafe fn unsafe_fn<T>() -> T {
    unimplemented!();
}

