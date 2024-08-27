pub trait MyTrait {
    type Assoc;
}

pub fn foo<S, T>(_s: S, _t: T)
where
    S: MyTrait,
    T: MyTrait<Assoc == S::Assoc>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
{
}

fn main() {}

