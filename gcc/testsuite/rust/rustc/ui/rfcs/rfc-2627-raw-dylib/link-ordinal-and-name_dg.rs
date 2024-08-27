#[link(name="foo")]
extern "C" {
    #[link_name="foo"]
    #[link_ordinal(42)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo();
    #[link_name="foo"]
    #[link_ordinal(5)]
// { dg-error "" "" { target *-*-* } .-1 }
    static mut imported_variable: i32;
}

fn main() {}

