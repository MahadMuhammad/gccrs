// Minimized test from #59311.

pub fn crash()
where
    for<'a> &'a (): 'static,
{
    || {};
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

