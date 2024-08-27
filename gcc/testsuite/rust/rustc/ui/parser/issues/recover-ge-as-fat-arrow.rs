//@ run-rustfix
fn main() {
    match 1 {
        1 >= {} // { dg-error "" "" { target *-*-* } }
        _ => { let _: u16 = 2u8; } // { dg-error ".E0308." "" { target *-*-* } }
    }
}

