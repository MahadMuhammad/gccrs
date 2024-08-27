trait Trait<T> {}

struct Bar(Box<dyn Trait<T>>);
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn main() {
    let x: Bar = unsafe { std::mem::transmute(()) };
}

