#[link(name = "foo")]
extern "C" {
    #[link_ordinal(3)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo();
}

#[link(name = "bar", kind = "static")]
extern "C" {
    #[link_ordinal(3)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn bar();
}

fn main() {}

