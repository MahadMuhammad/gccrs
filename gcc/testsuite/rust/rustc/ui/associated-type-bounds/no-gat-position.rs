// Test for <https://github.com/rust-lang/rust/issues/119857>.

pub trait Iter {
    type Item<'a>: 'a where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a, As1: Copy>>;
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }
}

impl Iter for () {
    type Item<'a> = &'a mut [()];

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> { None }
}

fn main() {}

