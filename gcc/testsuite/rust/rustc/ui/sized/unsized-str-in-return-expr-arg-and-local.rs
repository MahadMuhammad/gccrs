fn foo() -> impl Sized {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
    *"" // { help "" "" { target *-*-* } }
}
fn bar(_: impl Sized) {}
struct S;

impl S {
    fn baz(&self, _: impl Sized) {}
}

fn main() {
    let _ = foo();
    let x = *"";
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
// { help ".E0277." "" { target *-*-* } .-4 }
    bar(x);
    S.baz(x);
    bar(*"");
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
    S.baz(*"");
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
}

