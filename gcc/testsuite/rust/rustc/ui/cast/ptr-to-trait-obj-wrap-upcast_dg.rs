trait Super {}
trait Sub: Super {}

struct Wrapper<T: ?Sized>(T);

// This cast should not compile.
// Upcasting can't work here, because we are also changing the type (`Wrapper`),
// and reinterpreting would be confusing/surprising.
// See <https://github.com/rust-lang/rust/pull/120248#discussion_r1487739518>
fn cast(ptr: *const dyn Sub) -> *const Wrapper<dyn Super> {
    ptr as _ // { dg-error ".E0606." "" { target *-*-* } }
}

fn main() {}

