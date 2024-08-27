fn hi() -> impl Sized {
    std::ptr::null::<u8>()
}

fn main() {
    let b: Box<dyn Fn() -> Box<u8>> = Box::new(hi);
// { dg-error ".E0271." "" { target *-*-* } .-1 }
    let boxed = b();
    let null = *boxed;
    println!("{null:?}");
}

