// Regression test of #86162.

fn foo(x: impl Clone) {}
fn gen<T>() -> T { todo!() }

fn main() {
    foo(gen()); //<- Do not suggest `foo::<impl Clone>()`!
// { dg-error ".E0283." "" { target *-*-* } .-1 }
}

