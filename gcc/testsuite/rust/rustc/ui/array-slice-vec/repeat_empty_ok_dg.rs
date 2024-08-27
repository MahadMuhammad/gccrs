#![crate_type = "lib"]

pub struct Header<'a> {
    pub value: &'a [u8],
}

pub fn test() {
    let headers = [Header{value: &[]}; 128];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

pub fn test2() {
    let headers = [Header{value: &[0]}; 128];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

