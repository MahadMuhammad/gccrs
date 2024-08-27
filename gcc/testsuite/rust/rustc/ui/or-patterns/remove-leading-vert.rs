// Test the suggestion to remove a leading, or trailing `|`.

//@ run-rustfix

#![allow(warnings)]

fn main() {}

#[cfg(FALSE)]
fn leading() {
    fn fun1( | A: E) {} // { dg-error "" "" { target *-*-* } }
    fn fun2( || A: E) {} // { dg-error "" "" { target *-*-* } }
    let ( | A): E;
    let ( || A): (E); // { dg-error "" "" { target *-*-* } }
    let ( | A,): (E,);
    let [ | A ]: [E; 1];
    let [ || A ]: [E; 1]; // { dg-error "" "" { target *-*-* } }
    let TS( | A ): TS;
    let TS( || A ): TS; // { dg-error "" "" { target *-*-* } }
    let NS { f: | A }: NS;
    let NS { f: || A }: NS; // { dg-error "" "" { target *-*-* } }
}

#[cfg(FALSE)]
fn trailing() {
    let ( A | ): E; // { dg-error "" "" { target *-*-* } }
    let (a |,): (E,); // { dg-error "" "" { target *-*-* } }
    let ( A | B | ): E; // { dg-error "" "" { target *-*-* } }
    let [ A | B | ]: [E; 1]; // { dg-error "" "" { target *-*-* } }
    let S { f: B | }; // { dg-error "" "" { target *-*-* } }
    let ( A || B | ): E; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    match A {
        A | => {} // { dg-error "" "" { target *-*-* } }
        A || => {} // { dg-error "" "" { target *-*-* } }
        A || B | => {} // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
        | A | B | => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }

    // These test trailing-vert in `let` bindings, but they also test that we don't emit a
    // duplicate suggestion that would confuse rustfix.

    let a | : u8 = 0; // { dg-error "" "" { target *-*-* } }
    let a | = 0; // { dg-error "" "" { target *-*-* } }
    let a | ; // { dg-error "" "" { target *-*-* } }
}

