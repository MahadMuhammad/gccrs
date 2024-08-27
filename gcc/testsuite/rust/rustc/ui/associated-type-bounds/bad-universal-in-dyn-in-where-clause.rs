trait B {
    type AssocType;
}

fn f()
where
    dyn for<'j> B<AssocType: 'j>:,
// { dg-error "" "" { target *-*-* } .-1 }
{
}

fn main() {}

