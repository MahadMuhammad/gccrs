// test that "error: arguments for inline assembly must be copyable" doesn't show up in this code
//@ needs-asm-support
//@ only-x86_64
fn main() {
    let peb: *const PEB; // { dg-error ".E0412." "" { target *-*-* } }
    unsafe { std::arch::asm!("mov {0}, fs:[0x30]", out(reg) peb); }
}

