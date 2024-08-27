#[link(name = "foo")]
extern "C" {
    #[link_ordinal(72436)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo();
    #[link_ordinal(72436)]
// { dg-error "" "" { target *-*-* } .-1 }
    static mut imported_variable: i32;
}

fn main() {}

