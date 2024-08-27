trait Filter {
    type ToMatch;
}

impl<T> Filter for T // { dg-error ".E0275." "" { target *-*-* } }
where
    T: Fn(Self::ToMatch),
{
}

struct JustFilter<F: Filter> {
    filter: F,
}

fn main() {}

