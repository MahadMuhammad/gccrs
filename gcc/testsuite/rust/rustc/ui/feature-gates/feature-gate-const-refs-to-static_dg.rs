static S: i32 = 0;
static mut S_MUT: i32 = 0;

const C1: &i32 = &S; // { dg-error ".E0658." "" { target *-*-* } }
const C1_READ: () = {
    assert!(*C1 == 0);
};
const C2: *const i32 = unsafe { std::ptr::addr_of!(S_MUT) }; // { dg-error ".E0658." "" { target *-*-* } }

fn main() {
}

