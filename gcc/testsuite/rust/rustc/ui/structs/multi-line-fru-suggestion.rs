#[derive(Default)]
struct Inner {
    a: u8,
    b: u8,
}

#[derive(Default)]
struct Outer {
    inner: Inner,
    defaulted: u8,
}

fn main(){
    Outer {
// { dg-error ".E0063." "" { target *-*-* } .-1 }
        inner: Inner {
            a: 1,
            b: 2,
        }
        ..Default::default()
    };
}

