const FOO: *const u32 = {
    let x;
    &x // { dg-error ".E0381." "" { target *-*-* } }
};

fn main() {
    let FOO = FOO;
// { dg-error "" "" { target *-*-* } .-1 }
}

