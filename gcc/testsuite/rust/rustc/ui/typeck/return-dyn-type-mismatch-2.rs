trait Trait<T> {}

fn foo<T>() -> dyn Trait<T>
where
    dyn Trait<T>: Sized, // pesky sized predicate
{
    42
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

