struct S;

trait D {
    type P<T: Copy>;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

impl D for S {
    type P<T: Copy> = ();
}

fn main() {
    let _: <S as D>::P<String>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
}

