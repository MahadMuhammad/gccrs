//@ run-rustfix

pub fn f() -> String {  // { dg-error ".E0308." "" { target *-*-* } }
    0u8;
    "bla".to_string();
}

pub fn g() -> String {  // { dg-error ".E0308." "" { target *-*-* } }
    "this won't work".to_string();
    "removeme".to_string();
}

pub fn macro_tests() -> u32 {  // { dg-error ".E0308." "" { target *-*-* } }
    macro_rules! mac {
        () => (1);
    }
    mac!();
}

fn main() {}

