// ICE 'broken MIR: bad assignment: NoSolution'
// on trait with default method and no impls
// issue: rust-lang/rust#109869

trait Empty<T> {}

impl<T> Default for dyn Empty<T>
where
    Self: Sized,
{
    fn default() -> Self {
        ()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

pub fn main() {}

