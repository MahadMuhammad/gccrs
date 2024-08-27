enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit();
// { dg-error ".E0618." "" { target *-*-* } .-2 }
}

