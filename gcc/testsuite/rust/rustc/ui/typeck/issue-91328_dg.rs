// Regression test for issue #91328.

//@ run-rustfix

#![allow(dead_code)]

fn foo(r: Result<Vec<i32>, i32>) -> i32 {
    match r {
// { help "" "" { target *-*-* } .-1 }
        Ok([a, b]) => a + b,
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { dg-note ".E0529." "" { target *-*-* } .-2 }
        _ => 42,
    }
}

fn bar(o: Option<Vec<i32>>) -> i32 {
    match o {
// { help "" "" { target *-*-* } .-1 }
        Some([a, b]) => a + b,
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { dg-note ".E0529." "" { target *-*-* } .-2 }
        _ => 42,
    }
}

fn baz(v: Vec<i32>) -> i32 {
    match v {
// { help "" "" { target *-*-* } .-1 }
        [a, b] => a + b,
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { dg-note ".E0529." "" { target *-*-* } .-2 }
        _ => 42,
    }
}

fn qux(a: &Option<Box<[i32; 2]>>) -> i32 {
    match a {
// { help "" "" { target *-*-* } .-1 }
        Some([a, b]) => a + b,
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { dg-note ".E0529." "" { target *-*-* } .-2 }
        _ => 42,
    }
}

fn main() {}

