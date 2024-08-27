fn foo() {
    oops;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

unsafe fn bar() {
    std::mem::transmute::<_, *mut _>(1_u8);
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}

fn main() {}

