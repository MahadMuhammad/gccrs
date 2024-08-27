//@ normalize-stderr-test: "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//@ normalize-stderr-test: "([0-9a-f][0-9a-f] |╾─*ALLOC[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> "HEX_DUMP"
#![feature(const_refs_to_static)]
#![allow(static_mut_refs)]

fn invalid() {
    static S: i8 = 10;

    const C: &bool = unsafe { std::mem::transmute(&S) };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

    // This must be rejected here (or earlier), since it's not a valid `&bool`.
    match &true {
        C => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

fn extern_() {
    extern "C" {
        static S: i8;
    }

    const C: &i8 = unsafe { &S };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

    // This must be rejected here (or earlier), since the pattern cannot be read.
    match &0 {
        C => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

fn mutable() {
    static mut S_MUT: i32 = 0;

    const C: &i32 = unsafe { &S_MUT };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

    // This *must not build*, the constant we are matching against
    // could change its value!
    match &42 {
        C => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

fn main() {}

