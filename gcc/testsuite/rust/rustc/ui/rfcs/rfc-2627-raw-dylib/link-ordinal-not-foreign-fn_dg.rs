#[link_ordinal(123)]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo {}

#[link_ordinal(123)]
// { dg-error "" "" { target *-*-* } .-1 }
fn test() {}

#[link_ordinal(42)]
// { dg-error "" "" { target *-*-* } .-1 }
static mut imported_val: i32 = 123;

#[link(name = "exporter", kind = "raw-dylib")]
extern {
    #[link_ordinal(13)]
    fn imported_function();

    #[link_ordinal(42)]
    static mut imported_variable: i32;
}

fn main() {}

