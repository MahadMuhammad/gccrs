pub enum ErrorHandled {
    Reported,
    TooGeneric,
}

impl ErrorHandled {
    pub fn assert_reported(self) {
        match self {
// { dg-note "" "" { target *-*-* } .-1 }
            ErrorHandled::Reported => {}}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            ErrorHandled::TooGeneric => panic!(),
        }
    }
}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

