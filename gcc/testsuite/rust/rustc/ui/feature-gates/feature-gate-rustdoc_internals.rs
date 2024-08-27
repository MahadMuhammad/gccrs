#[doc(keyword = "match")] // { dg-error ".E0658." "" { target *-*-* } }
/// wonderful
mod foo {}

trait Mine {}

#[doc(fake_variadic)]  // { dg-error ".E0658." "" { target *-*-* } }
impl<T> Mine for (T,) {}

fn main() {}

