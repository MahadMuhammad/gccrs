// { dg-additional-options "-frust-edition=2021" }

// Test that we point to the correct location that results a union being captured.
// Union is special because it can't be disjointly captured.

union A {
    y: u32,
    x: (),
}

fn main() {
    let mut a = A { y: 1 };
    let mut c = || {
// { dg-error "" "" { target *-*-* } .-1 }
        let _ = unsafe { &a.y };
        let _ = &mut a;
// { dg-error "" "" { target *-*-* } .-1 }
        let _ = unsafe { &mut a.y };
    };
    a.y = 1;
// { dg-error ".E0506." "" { target *-*-* } .-1 }
// { dg-error ".E0506." "" { target *-*-* } .-2 }
    c();
// { dg-error "" "" { target *-*-* } .-1 }
}

