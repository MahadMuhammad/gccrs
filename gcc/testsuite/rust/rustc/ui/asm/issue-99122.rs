//@ needs-asm-support
//@ only-x86_64

pub unsafe fn test() {
    let pointer = 1u32 as *const _;
// { dg-error ".E0641." "" { target *-*-* } .-1 }
    core::arch::asm!(
        "nop",
        in("eax") pointer,
    );
}

fn main() {}

