const FOO: *const u32 = {
// { dg-error "" "" { target *-*-* } .-1 }
    let x = 42;
    &x
};

union Union {
    ptr: *const u32,
}

const BAR: Union = {
// { dg-error "" "" { target *-*-* } .-1 }
    let x = 42;
    Union { ptr: &x }
};

const BAZ: Union = {
// { dg-error "" "" { target *-*-* } .-1 }
    let x = 42_u32;
    Union { ptr: &(&x as *const u32) as *const *const u32 as _ }
};

const FOOMP: *const u32 = {
// { dg-error "" "" { target *-*-* } .-1 }
    let x = 42_u32;
    &(&x as *const u32) as *const *const u32 as _
};

fn main() {}

