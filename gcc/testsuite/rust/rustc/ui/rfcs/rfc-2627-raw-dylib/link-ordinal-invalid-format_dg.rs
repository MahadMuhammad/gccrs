#[link(name = "foo")]
extern "C" {
    #[link_ordinal("JustMonika")]
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo();
    #[link_ordinal("JustMonika")]
// { dg-error "" "" { target *-*-* } .-1 }
    static mut imported_variable: i32;
}

fn main() {}

