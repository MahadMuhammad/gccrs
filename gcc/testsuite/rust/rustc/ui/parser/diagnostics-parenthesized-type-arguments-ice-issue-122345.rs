fn main() {
    unsafe {
        dealloc(ptr2, Layout::(x: !)(1, 1)); // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

