fn fmt(it: &(std::cell::Cell<Option<impl FnOnce()>>,)) {
    (it.0.take())()
// { dg-error ".E0618." "" { target *-*-* } .-1 }
}

fn main() {}

