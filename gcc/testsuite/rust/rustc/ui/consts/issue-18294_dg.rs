fn main() {
    const X: u32 = 1;
    const Y: usize = unsafe { &X as *const u32 as usize }; // { dg-error "" "" { target *-*-* } }
    println!("{}", Y);
}

