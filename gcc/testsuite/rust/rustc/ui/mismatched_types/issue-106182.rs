//@ run-rustfix

struct _S(u32, Vec<i32>);

fn _foo(x: &_S) {
    match x {
        _S(& (mut _y), _v) => {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {
}

