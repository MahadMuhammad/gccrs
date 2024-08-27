// Checks that derived implementations of Clone and Debug do not
// contribute to dead code analysis (issue #84647).

#![forbid(dead_code)]

struct A { f: () }
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(Clone)]
struct B { f: () }
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(Debug)]
struct C { f: () }
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(Debug,Clone)]
struct D { f: () }
// { dg-error "" "" { target *-*-* } .-1 }

struct E { f: () }
// { dg-error "" "" { target *-*-* } .-1 }
// Custom impl, still doesn't read f
impl Clone for E {
    fn clone(&self) -> Self {
        Self { f: () }
    }
}

struct F { f: () }
// Custom impl that actually reads f
impl Clone for F {
    fn clone(&self) -> Self {
        Self { f: self.f }
    }
}

fn main() {
    let _ = A { f: () };
    let _ = B { f: () };
    let _ = C { f: () };
    let _ = D { f: () };
    let _ = E { f: () };
    let _ = F { f: () };
}

