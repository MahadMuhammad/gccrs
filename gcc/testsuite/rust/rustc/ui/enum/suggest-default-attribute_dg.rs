pub enum Test { // { help "" "" { target *-*-* } }
    #[default]
// { dg-error "" "" { target *-*-* } .-1 }
    First,
    Second,
}

fn main() {}

