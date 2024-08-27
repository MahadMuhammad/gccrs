trait Lam {}

pub struct B;
impl Lam for B {}
pub struct Wrap<T>(T);

const _A: impl Lam = {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    let x: Wrap<impl Lam> = Wrap(B);
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    x.0
};

fn main() {}

