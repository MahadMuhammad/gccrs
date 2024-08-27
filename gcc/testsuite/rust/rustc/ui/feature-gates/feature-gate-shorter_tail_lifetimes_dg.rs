fn f() -> usize {
    let c = std::cell::RefCell::new("..");
    c.borrow().len() // { dg-error ".E0597." "" { target *-*-* } }
}

fn main() {
    let _ = f();
}

