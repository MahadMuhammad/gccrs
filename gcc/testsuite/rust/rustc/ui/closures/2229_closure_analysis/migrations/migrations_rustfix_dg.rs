//@ run-rustfix
#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

// Test the two possible cases for automated migartion using rustfix
// - Closure contains a block i.e.  `|| { .. };`
// - Closure contains just an expr `|| ..;`

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

fn closure_contains_block() {
    let t = (Foo(0), Foo(0));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

fn closure_doesnt_contain_block() {
    let t = (Foo(0), Foo(0));
    let c = || t.0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {
    closure_contains_block();
    closure_doesnt_contain_block();
}

