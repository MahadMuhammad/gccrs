struct NeedsDropTypes<'tcx, F>(std::marker::PhantomData<&'tcx F>);

impl<'tcx, F, I> Iterator for NeedsDropTypes<'tcx, F>
// { dg-error ".E0046." "" { target *-*-* } .-1 }
where
    F: Fn(&Missing) -> Result<I, ()>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    I: Iterator<Item = Missing>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{}

fn main() {}

