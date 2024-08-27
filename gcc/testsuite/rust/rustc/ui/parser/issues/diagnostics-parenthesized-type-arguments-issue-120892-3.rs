struct Foo(u32, u32);
impl Foo {
    fn foo(&self) {
        match *self {
            Foo::(1, 2) => {}, // { help "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
             _ => {},
        }
    }
}

fn main() {}

