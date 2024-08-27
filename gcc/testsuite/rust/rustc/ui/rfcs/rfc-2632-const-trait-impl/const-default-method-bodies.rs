#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

#[const_trait]
trait ConstDefaultFn: Sized {
    fn b(self);

    fn a(self) {
        self.b();
    }
}

struct NonConstImpl;
struct ConstImpl;

impl ConstDefaultFn for NonConstImpl {
    fn b(self) {}
}

impl const ConstDefaultFn for ConstImpl {
    fn b(self) {}
}

const fn test() {
    NonConstImpl.a();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    ConstImpl.a();
}

fn main() {}

