pub trait TraitEngine<'tcx>: 'tcx {}

pub trait TraitEngineExt<'tcx> {
    fn register_predicate_obligations(&mut self);
}

impl<T: ?Sized + TraitEngine<'tcx>> TraitEngineExt<'tcx> for T {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }
    fn register_predicate_obligations(&mut self) {}
}

fn main() {}

