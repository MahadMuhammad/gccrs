struct S<'a>(&'a str);

fn f(inner: fn(&str, &S)) {
}

#[allow(unreachable_code)]
fn main() {
    let inner: fn(_, _) = unimplemented!();
    f(inner); // { dg-error ".E0308." "" { target *-*-* } }
}

