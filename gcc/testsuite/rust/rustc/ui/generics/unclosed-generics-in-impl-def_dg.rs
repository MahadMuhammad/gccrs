impl<S: Into<std::borrow::Cow<'static, str>> From<S> for Canonical {} // { dg-error "" "" { target *-*-* } }
fn main() {}

