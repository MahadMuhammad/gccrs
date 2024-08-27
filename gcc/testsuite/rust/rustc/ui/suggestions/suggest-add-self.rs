struct X(i32);

impl X {
    pub(crate) fn f() {
        self.0
// { dg-error ".E0424." "" { target *-*-* } .-1 }
    }

    pub fn g() {
        self.0
// { dg-error ".E0424." "" { target *-*-* } .-1 }
    }
}

fn main() {}

