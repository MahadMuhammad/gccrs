// This test tests the precedence of `|` (or-patterns) undelimited nested patterns. In particular,
// we want to reserve the syntactic space of a pattern followed by a type annotation for possible
// future type ascription, so we need to make sure that any time a pattern is followed by type
// annotation (for now), the pattern is not a top-level or-pattern. However, there are also a few
// types of patterns that allow undelimited subpatterns that could cause the same ambiguity.
// Currently, those should be impossible due to precedence rule. This test enforces that.

enum E {
    A,
    B,
}

fn foo() {
    use E::*;

    // ok
    let b @ (A | B): E = A;

    let b @ A | B: E = A; // { dg-error ".E0408." "" { target *-*-* } }
// { dg-error ".E0408." "" { target *-*-* } .-1 }
}

enum F {
    A(usize),
    B(usize),
}

fn bar() {
    use F::*;

    // ok
    let (A(x) | B(x)): F = A(3);

    let &A(_) | B(_): F = A(3); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let &&A(_) | B(_): F = A(3); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let &mut A(_) | B(_): F = A(3); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let &&mut A(_) | B(_): F = A(3); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

