// issue #117766

#![feature(let_chains)]
fn main() {
    if let () = ()
        && let () = () { // { dg-error "" "" { target *-*-* } }
        && let () = ()
    {
    }
}

fn quux() {
    while let () = ()
        && let () = () { // { dg-error "" "" { target *-*-* } }
        && let () = ()
    {
    }
}

fn foobar() {
    while false {}
    {
        && let () = ()
}

fn fubar() {
    while false {
        {
            && let () = ()
    }
}

fn qux() {
    let foo = false;
    match foo {
        _ if foo => {
            && let () = ()
        _ => {}
    }
}

fn foo() {
    {
    && let () = ()
}

fn bar() {
    if false {}
    {
        && let () = ()
}

fn baz() {
    if false {
        {
            && let () = ()
    }
} // { dg-error "" "" { target *-*-* } }

