// Issue #117720

#![feature(let_chains)]

fn main() {
    if let () = ()
        && let () = (); // { dg-error "" "" { target *-*-* } }
        && let () = ()
    {
    }
}

fn foo() {
    if let () = ()
        && () == (); // { dg-error "" "" { target *-*-* } }
        && 1 < 0
    {
    }
}

fn bar() {
    if let () = ()
        && () == (); // { dg-error "" "" { target *-*-* } }
        && let () = ()
    {
    }
}

