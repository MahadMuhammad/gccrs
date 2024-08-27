macro_rules! m {
    ($p: path) => (pub(in $p) struct Z;)
}

struct S<T>(T);
m!{ S<u8> } // { dg-error ".E0433." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

mod m {
    m!{ m<> } // { dg-error "" "" { target *-*-* } }
}

fn main() {}

