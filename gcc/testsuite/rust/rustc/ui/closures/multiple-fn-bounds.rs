fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    todo!();
}

fn main() {
    let v = true;
    foo(move |x| v);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { dg-note ".E0631." "" { target *-*-* } .-2 }
// { dg-note ".E0631." "" { target *-*-* } .-3 }
// { dg-note ".E0631." "" { target *-*-* } .-4 }
}

