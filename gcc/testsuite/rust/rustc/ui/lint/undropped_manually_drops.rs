//@ check-fail

struct S;

fn main() {
    let mut manual1 = std::mem::ManuallyDrop::new(S);
    let mut manual2 = std::mem::ManuallyDrop::new(S);
    let mut manual3 = std::mem::ManuallyDrop::new(S);

    drop(std::mem::ManuallyDrop::new(S)); // { dg-error "" "" { target *-*-* } }
    drop(manual1); // { dg-error "" "" { target *-*-* } }
    drop({ manual3 }); // { dg-error "" "" { target *-*-* } }

    // These lines will drop `S` and should be okay.
    unsafe {
        std::mem::ManuallyDrop::drop(&mut std::mem::ManuallyDrop::new(S));
        std::mem::ManuallyDrop::drop(&mut manual2);
    }
}

