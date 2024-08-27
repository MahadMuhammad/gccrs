#[deny(unused_imports)]
mod rank {
    pub use self::Professor::*;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    pub use self::Lieutenant::{JuniorGrade, Full};
// { dg-error ".E0364." "" { target *-*-* } .-1 }
// { dg-error ".E0364." "" { target *-*-* } .-2 }
// { dg-error ".E0364." "" { target *-*-* } .-3 }
    pub use self::PettyOfficer::*;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    pub use self::Crewman::*;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    enum Professor {
        Adjunct,
        Assistant,
        Associate,
        Full
    }

    enum Lieutenant {
        JuniorGrade,
        Full,
    }

    pub(in rank) enum PettyOfficer {
        SecondClass,
        FirstClass,
        Chief,
        MasterChief
    }

    pub(crate) enum Crewman {
        Recruit,
        Apprentice,
        Full
    }

}

fn main() {}

