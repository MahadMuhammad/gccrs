trait FromResidual<R = <Self as Try>::Residual> {
    fn from_residual(residual: R) -> Self;
}

trait Try {
    type Residual;
}

fn w<'a, T: 'a, F: Fn(&'a T)>() {
    let b: &dyn FromResidual = &();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}

fn main() {}

