// Strip out raw byte dumps to make comparison platform-independent:
//@ normalize-stderr-test: "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//@ normalize-stderr-test: "([0-9a-f][0-9a-f] |╾─*ALLOC[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> "HEX_DUMP"
#![feature(never_type)]

const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) }; // { dg-error ".E0080." "" { target *-*-* } }
const _: &[!; 0] = unsafe { &*(1_usize as *const [!; 0]) }; // ok
const _: &[!] = unsafe { &*(1_usize as *const [!; 0]) }; // ok
const _: &[!] = unsafe { &*(1_usize as *const [!; 1]) }; // { dg-error ".E0080." "" { target *-*-* } }
const _: &[!] = unsafe { &*(1_usize as *const [!; 42]) }; // { dg-error ".E0080." "" { target *-*-* } }

fn main() {}

