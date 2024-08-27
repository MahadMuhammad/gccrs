#![deny(break_with_label_and_loop)]

macro_rules! foo {
    ( $f:block ) => {
        '_l: loop {
            break '_l $f; // { dg-error "" "" { target *-*-* } }
        }
    };
}

fn main() {
    let x = foo!({ 3 });
}

