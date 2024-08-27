static X: i32 = 1;
const C: i32 = 2;

const CR: &'static mut i32 = &mut C; // { dg-error ".E0764." "" { target *-*-* } }

// { dg-warning "" "" { target *-*-* } .-3 }
static STATIC_REF: &'static mut i32 = &mut X; // { dg-error ".E0658." "" { target *-*-* } }

static CONST_REF: &'static mut i32 = &mut C; // { dg-error ".E0764." "" { target *-*-* } }

// { dg-warning "" "" { target *-*-* } .-3 }

fn main() {}

