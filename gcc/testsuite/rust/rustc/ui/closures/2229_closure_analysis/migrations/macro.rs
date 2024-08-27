//@ run-rustfix

// See https://github.com/rust-lang/rust/issues/87955

#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }


#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

fn main() {
    let a = (Foo(0), Foo(1));
    let _ = || dbg!(a.0);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
}
// { dg-note "" "" { target *-*-* } .-1 }

