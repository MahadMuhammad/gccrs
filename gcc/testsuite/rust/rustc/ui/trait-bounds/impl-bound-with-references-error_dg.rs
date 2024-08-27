// Regression test for #105138.
// This test ensures that the compiler does not add note
// for implementation of trait whose inner type is erroneous.

pub enum LabelText {
    Plain,
}

impl<T> From<T> for LabelText
// { dg-error ".E0119." "" { target *-*-* } .-1 }
where
    T: Into<Cow<'static, str>>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
    fn from(text: T) -> Self {
        LabelText::Plain(text.into()) // { dg-error ".E0618." "" { target *-*-* } }
    }
}

fn main() {}

