//@ run-rustfix

#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

struct ConstainsDropField(Foo, Foo);

// Test that lint is triggered if a path that implements Drop is not captured by move
fn test_precise_analysis_drop_paths_not_captured_by_move() {
    let t = ConstainsDropField(Foo(10), Foo(20));

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _t = &t.1;
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

struct S;
impl Drop for S {
    fn drop(&mut self) {}
}

struct T(S, S);
struct U(T, T);

// Test precise analysis for the lint works with paths longer than one.
fn test_precise_analysis_long_path_missing() {
    let u = U(T(S, S), T(S, S));

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _x = u.0.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _x = u.0.1;
// { dg-note "" "" { target *-*-* } .-1 }
        let _x = u.1.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }


fn main() {
    test_precise_analysis_drop_paths_not_captured_by_move();
    test_precise_analysis_long_path_missing();
}

