const fn bool_cast(ptr: *const bool) { unsafe {
    let _val = *ptr as u32; // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}}

const _: () = {
    let v = 3_u8;
    bool_cast(&v as *const u8 as *const bool);
};

fn main() {}

