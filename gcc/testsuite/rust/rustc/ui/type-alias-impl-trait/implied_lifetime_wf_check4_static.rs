#![feature(type_alias_impl_trait)]

mod test_type_param_static {
    pub type Ty<A> = impl Sized + 'static;
// { dg-error ".E0310." "" { target *-*-* } .-1 }
    fn defining<A: 'static>(s: A) -> Ty<A> {
        s
    }
    pub fn assert_static<A: 'static>() {}
}
use test_type_param_static::*;

fn test<A>()
where
    Ty<A>: 'static,
{
    assert_static::<A>()
// { dg-error ".E0310." "" { target *-*-* } .-1 }
}

fn main() {}

