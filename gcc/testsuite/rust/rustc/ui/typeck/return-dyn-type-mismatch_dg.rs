pub trait TestTrait {
    type MyType;

    fn func() -> Option<Self>
    where
        Self: Sized;
}

impl<T> dyn TestTrait<MyType = T>
where
    Self: Sized, // pesky sized predicate
{
    fn other_func() -> dyn TestTrait<MyType = T> {
        match Self::func() {
            None => None,
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

