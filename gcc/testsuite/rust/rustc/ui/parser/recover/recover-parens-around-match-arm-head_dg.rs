//@ run-rustfix
fn main() {
    let val = 42;
    let x = match val {
        (0 if true) => {
// { dg-error "" "" { target *-*-* } .-1 }
            42u8
        }
        _ => 0u8,
    };
    let _y: u32 = x; // { dg-error ".E0308." "" { target *-*-* } }
}

