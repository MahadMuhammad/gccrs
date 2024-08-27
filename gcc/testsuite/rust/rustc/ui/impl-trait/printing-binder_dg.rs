trait Trait<'a> {}
impl<T> Trait<'_> for T {}
fn whatever() -> impl for<'a> Trait<'a> + for<'b> Trait<'b> {}

fn whatever2() -> impl for<'c> Fn(&'c ()) {
    |_: &()| {}
}

fn main() {
    let x: u32 = whatever();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let x2: u32 = whatever2();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

