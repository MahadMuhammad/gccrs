// ICE 'broken MIR: bad assignment: NoSolution'
// on trait with default method and no impls
// issue: rust-lang/rust#109869

type Spanned<T> = (T, ());

trait Span<T> {}

impl<T> Span<T> for (T, ()) {}

impl<F, T: From<F>> From<Spanned<F>> for dyn Span<T>
where
    Self: Sized
{
    fn from((from, ()): Spanned<F>) -> Self {
        (T::from(from), ())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

pub fn main() {}

