// issue: rust-lang/rust#104779
// ICE region infer, IndexMap: key not found

struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Subtype = Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>;
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo() -> impl Sized {
// { dg-warning "" "" { target *-*-* } .-1 }
    loop {
        match foo() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
            Subtype::Bar => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
            Supertype::Var(x) => {}
        }
    }
}

pub fn main() {}

