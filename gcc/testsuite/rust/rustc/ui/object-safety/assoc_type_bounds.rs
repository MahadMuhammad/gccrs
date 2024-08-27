trait Foo<T> {
    type Bar
    where
        T: Cake;
}

trait Cake {}
impl Cake for () {}

fn foo(_: &dyn Foo<()>) {} // { dg-error ".E0191." "" { target *-*-* } }
fn bar(_: &dyn Foo<i32>) {} // { dg-error ".E0191." "" { target *-*-* } }

fn main() {}

