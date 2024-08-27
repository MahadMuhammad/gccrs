// issue: rust-lang/rust#101852
// ICE opaque type with non-universal region substs

pub fn ice(x: impl AsRef<str>) -> impl IntoIterator<Item = ()> {
// { dg-warning "" "" { target *-*-* } .-1 }
    vec![].append(&mut ice(x.as_ref()));
// { dg-error ".E0792." "" { target *-*-* } .-1 }

    Vec::new()
}

fn main() {}

