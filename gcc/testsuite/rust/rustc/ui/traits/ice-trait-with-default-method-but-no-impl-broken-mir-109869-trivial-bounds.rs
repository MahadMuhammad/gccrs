// ICE 'broken MIR: bad assignment: NoSolution'
// on trait with default method and no impls
// issue: rust-lang/rust#109869

#![feature(trivial_bounds)]
trait Empty {}

impl Default for dyn Empty
where
    Self: Sized,
{
    fn default() -> Self {
        ()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

pub fn main() {}

