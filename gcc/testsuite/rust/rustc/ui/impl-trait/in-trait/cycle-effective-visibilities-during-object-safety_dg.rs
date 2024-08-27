trait Marker {}
impl Marker for u32 {}

trait MyTrait {
    fn foo(&self) -> impl Marker;
}

struct Outer;

impl MyTrait for Outer {
    fn foo(&self) -> impl Marker {
        42
    }
}

impl dyn MyTrait {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
    fn other(&self) -> impl Marker {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
        MyTrait::foo(&self)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
    }
}

fn main() {}

