//@ check-pass

trait Trait<'a> {}

fn add_auto<'a>(x: *mut dyn Trait<'a>) -> *mut (dyn Trait<'a> + Send) {
    x as _
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

// (to test diagnostic list formatting)
fn add_multiple_auto<'a>(x: *mut dyn Trait<'a>) -> *mut (dyn Trait<'a> + Send + Sync + Unpin) {
    x as _
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

fn main() {}

