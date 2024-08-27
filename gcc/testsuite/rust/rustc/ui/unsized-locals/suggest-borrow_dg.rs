fn main() {
    let x: [u8] = vec!(1, 2, 3)[..]; // { dg-error ".E0277." "" { target *-*-* } }
    let x: &[u8] = vec!(1, 2, 3)[..]; // { dg-error ".E0308." "" { target *-*-* } }
    let x: [u8] = &vec!(1, 2, 3)[..]; // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let x: &[u8] = &vec!(1, 2, 3)[..];
}

