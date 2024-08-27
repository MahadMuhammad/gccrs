#[allow(dead_code)]
fn foo<M>(_m: M)
where
    M::Item: Temp,
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
{
}

fn main() {}

