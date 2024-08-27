fn main() {
    let mut buf = [0u8; 50];
    let mut bref = buf.as_slice();
    foo(&mut bref);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn foo(_: &mut impl std::io::Write) {}

