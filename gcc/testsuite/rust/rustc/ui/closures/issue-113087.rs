fn some_fn<'a>(_: &'a i32, _: impl FnOnce(&'a i32)) {}

fn main() {
    let some_closure = |_| {};

    for a in [1] {
        some_fn(&a, |c| { // { dg-error ".E0597." "" { target *-*-* } }
            some_closure(c);
        });
    }
}

