trait Monad {
    type Unwrapped;
    type Wrapped<B>;

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        todo!()
    }
}

fn join<MOuter, MInner, A>(outer: MOuter) -> MOuter::Wrapped<A>
where
    MOuter: Monad<Unwrapped = MInner>,
    MInner: Monad<Unwrapped = A, Wrapped = MOuter::Wrapped<A>>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
{
    outer.bind(|inner| inner)
}

fn main() {
    assert_eq!(join(Some(Some(true))), Some(true));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

