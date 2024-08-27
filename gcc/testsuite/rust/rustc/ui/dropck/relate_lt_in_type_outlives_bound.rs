struct Wrapper<'a, T>(&'a T)
where
    T: 'a;

impl<'a, T> Drop for Wrapper<'a, T>
where
    T: 'static,
// { dg-error ".E0367." "" { target *-*-* } .-1 }
{
    fn drop(&mut self) {}
}

fn main() {}

