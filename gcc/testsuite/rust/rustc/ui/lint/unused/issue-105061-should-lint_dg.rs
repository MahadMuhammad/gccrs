#![warn(unused)]
#![deny(warnings)]

struct Inv<'a>(#[allow(dead_code)] &'a mut &'a ());

trait Trait<'a> {}
impl<'b> Trait<'b> for for<'a> fn(Inv<'a>) {}

fn with_bound()
where
    for<'b> (for<'a> fn(Inv<'a>)): Trait<'b>, // { dg-error "" "" { target *-*-* } }
{}

trait Hello<T> {}
fn with_dyn_bound<T>()
where
    (dyn Hello<(for<'b> fn(&'b ()))>): Hello<T> // { dg-error "" "" { target *-*-* } }
{}

fn main() {
    with_bound();
    with_dyn_bound();
}

