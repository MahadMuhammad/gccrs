trait Trait {}

impl Trait for i32 {}

// Since `Assoc` doesn't actually exist, it's "stranded", and won't show up in
// the list of opaques that may be defined by the function. Make sure we don't
// ICE in this case.
fn produce<T>() -> impl Trait<Assoc = impl Trait> {
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
    16
}

fn main () {}

