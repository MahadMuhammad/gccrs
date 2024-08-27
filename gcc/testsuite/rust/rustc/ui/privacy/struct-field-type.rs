mod m {
    struct Priv;
    pub type Leak = Priv; // { dg-warning "" "" { target *-*-* } }
}

struct S {
    field: m::Leak, // { dg-error "" "" { target *-*-* } }
}

fn main() {}

