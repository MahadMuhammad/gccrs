#![deny(dead_code)]

struct UnusedStruct; // { dg-error "" "" { target *-*-* } }
impl UnusedStruct {
    fn unused_impl_fn_1() {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("blah");
    }

    fn unused_impl_fn_2(var: i32) {
        println!("foo {}", var);
    }

    fn unused_impl_fn_3(var: i32) {
        println!("bar {}", var);
    }
}

fn main() {}

