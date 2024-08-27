#![warn(unused_labels)]

fn main() {
    'while_loop: while true { // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
        while_loop;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
    'while_let: while let Some(_) = Some(()) {
// { dg-warning "" "" { target *-*-* } .-1 }
        while_let;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
    'for_loop: for _ in 0..3 {
// { dg-warning "" "" { target *-*-* } .-1 }
        for_loop;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
    'LOOP: loop {
// { dg-warning "" "" { target *-*-* } .-1 }
        LOOP;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
}

fn foo() {
    'LOOP: loop {
        break LOOP;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
    'while_loop: while true { // { dg-warning "" "" { target *-*-* } }
        break while_loop;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
    'while_let: while let Some(_) = Some(()) {
        break while_let;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
    'for_loop: for _ in 0..3 {
        break for_loop;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
}

fn bar() {
    let foo = ();
    'while_loop: while true { // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
        break foo;
// { dg-error ".E0571." "" { target *-*-* } .-1 }
    };
    'while_let: while let Some(_) = Some(()) {
// { dg-warning "" "" { target *-*-* } .-1 }
        break foo;
// { dg-error ".E0571." "" { target *-*-* } .-1 }
    }
    'for_loop: for _ in 0..3 {
// { dg-warning "" "" { target *-*-* } .-1 }
        break foo;
// { dg-error ".E0571." "" { target *-*-* } .-1 }
    };
}

