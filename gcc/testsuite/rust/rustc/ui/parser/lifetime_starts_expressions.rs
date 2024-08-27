#![allow(unused, dead_code)]

fn foo() -> u32 {
    return 'label: loop { break 'label 42; };
}

fn bar() -> u32 {
    loop { break 'label: loop { break 'label 42; }; }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn baz() -> u32 {
    'label: loop {
        break 'label
// { dg-warning "" "" { target *-*-* } .-1 }
            loop { break 42; };
// { help "" "" { target *-*-* } .-1 }
    };

    'label2: loop {
        break 'label2 'inner: loop { break 42; };
        // no warnings or errors here
    }
}

pub fn main() {
    // Regression test for issue #86948, as resolved in #87026:
    let a = 'first_loop: loop {
        break 'first_loop 1;
    };
    let b = loop {
        break 'inner_loop: loop {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
            break 'inner_loop 1;
        };
    };
}

