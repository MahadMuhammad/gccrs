fn main() {
    let x: i32 = 5;
    let ptr: *const i32 = &x;
    let dptr: **const i32 = &ptr;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

