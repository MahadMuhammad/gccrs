//@ compile-flags:--test

struct A {}

impl A {
    #[test]
// { dg-error "" "" { target *-*-* } .-1 }
    fn new() -> A {
        A {}
    }
    #[test]
// { dg-error "" "" { target *-*-* } .-1 }
    fn recovery_witness() -> A {
        A {}
    }
}

#[test]
fn test() {
    let _ = A::new();
}

fn main() {}

