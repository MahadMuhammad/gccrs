#[derive(Clone)]
struct ThingThatDoesAThing;

trait DoesAThing {}

impl DoesAThing for ThingThatDoesAThing {}

fn clones_impl_ref_inline(thing: &impl DoesAThing) {
// { help "" "" { target *-*-* } .-1 }
    drops_impl_owned(thing.clone()); // { dg-error ".E0277." "" { target *-*-* } }
// { dg-note ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
}

fn drops_impl_owned(_thing: impl DoesAThing) { }

fn main() {
    clones_impl_ref_inline(&ThingThatDoesAThing);
}

