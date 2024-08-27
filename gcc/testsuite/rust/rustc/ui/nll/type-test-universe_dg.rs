// Regression test for #98095: make sure that
// we detect that S needs to outlive 'static.

fn outlives_forall<T>()
where
    for<'u> T: 'u,
{
}

fn test1<S>() {
    outlives_forall::<S>();
// { dg-error "" "" { target *-*-* } .-1 }
}

struct Value<'a>(&'a ());
fn test2<'a>() {
    outlives_forall::<Value<'a>>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

