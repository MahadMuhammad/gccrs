//@ run-rustfix

#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {
    struct Foo(u32);
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped {}", self.0);
        }
    }

    let f0 = Foo(0);
    let f1 = Foo(1);

    let c0 = move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        let _ = f0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    let c1 = move || {
        let _ = &f1;
    };

    println!("dropping 0");
    drop(c0);
    println!("dropping 1");
    drop(c1);
    println!("dropped all");
}
// { dg-note "" "" { target *-*-* } .-1 }

