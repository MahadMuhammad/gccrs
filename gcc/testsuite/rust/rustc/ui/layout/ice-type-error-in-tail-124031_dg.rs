// Regression test for issue #124031
// Checks that we don't ICE when the tail
// of an ADT has a type error

trait Trait {
    type RefTarget;
}

impl Trait for () {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

struct Other {
    data: <() as Trait>::RefTarget,
}

fn main() {
    unsafe {
        std::mem::transmute::<Option<()>, Option<&Other>>(None);
    }
}

