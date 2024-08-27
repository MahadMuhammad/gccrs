struct Thing {
    state: u8,
}

impl Thing {
    fn oof(*mut Self) { // { dg-error "" "" { target *-*-* } }
        self.state = 1;
// { dg-error ".E0424." "" { target *-*-* } .-1 }
    }
}

fn main() {}

