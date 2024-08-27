use a::TyCtxt;

mod a {
    use std::ops::Deref;
    pub struct TyCtxt<'tcx> {
        gcx: &'tcx GlobalCtxt<'tcx>,
    }

    impl<'tcx> Deref for TyCtxt<'tcx> {
        type Target = &'tcx GlobalCtxt<'tcx>;

        fn deref(&self) -> &Self::Target {
            &self.gcx
        }
    }

    pub struct GlobalCtxt<'tcx> {
        pub sess: &'tcx Session,
        _t: &'tcx (),
    }

    pub struct Session {
        pub opts: (),
    }
}

mod b {
    fn foo<'tcx>(tcx: crate::TyCtxt<'tcx>) {
        tcx.opts;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { help ".E0609." "" { target *-*-* } .-2 }
    }
}

fn main() {}

