// `Rc` is not ever `Copy`, we should not suggest adding `T: Copy` constraint
fn duplicate_rc<T>(t: std::rc::Rc<T>) -> (std::rc::Rc<T>, std::rc::Rc<T>) {
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn main() {}

