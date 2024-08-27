type Lazy<T> = Box<dyn Fn() -> T + 'static>;

fn test(x: &i32) -> Lazy<i32> {
    Box::new(|| {
// { dg-error ".E0373." "" { target *-*-* } .-1 }
// { dg-error ".E0373." "" { target *-*-* } .-2 }
        *x
    })
}

fn main() {}

