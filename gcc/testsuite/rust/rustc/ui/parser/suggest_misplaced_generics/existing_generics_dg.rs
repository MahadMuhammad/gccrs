// Issue: 103366
// there is already an existing generic on f, so don't show a suggestion

#[allow(unused)]
fn<'a, B: 'a + std::ops::Add<Output = u32>> f<T>(_x: B) { }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

