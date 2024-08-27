// Regression test for issue #89396: Try to recover from a
// `=>` -> `=` or `->` typo in a match arm.

//@ run-rustfix

fn main() {
    let opt = Some(42);
    let _ = match opt {
        Some(_) = true,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        None -> false,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    };
}

