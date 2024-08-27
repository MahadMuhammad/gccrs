// Regression test for issue #121424
#[repr(C)]
struct MySlice<T: Copy>(bool, T);
type MySliceBool = MySlice<[bool]>;
const MYSLICE_GOOD: &MySliceBool = &MySlice(true, [false]);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn main() {}

