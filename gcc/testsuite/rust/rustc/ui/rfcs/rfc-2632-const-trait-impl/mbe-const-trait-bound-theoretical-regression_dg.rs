// Demonstrates and records a theoretical regressions / breaking changes caused by the
// introduction of const trait bounds.

// Setting the edition to 2018 since we don't regress `demo! { dyn const }` in Rust <2018.
// { dg-additional-options "-frust-edition=2018" }

macro_rules! demo {
    ($ty:ty) => { compile_error!("ty"); };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    (impl $c:ident Trait) => {};
    (dyn $c:ident Trait) => {};
}

demo! { impl const Trait }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

demo! { dyn const Trait }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

