fn main() {
    let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

