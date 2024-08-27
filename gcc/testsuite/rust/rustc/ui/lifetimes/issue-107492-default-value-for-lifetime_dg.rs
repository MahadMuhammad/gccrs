pub struct DefaultLifetime<'a, 'b = 'static> {
// { dg-error "" "" { target *-*-* } .-1 }
    _marker: std::marker::PhantomData<&'a &'b ()>,
}

fn main(){}

