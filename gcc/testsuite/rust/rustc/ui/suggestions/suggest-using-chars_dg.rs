pub fn main() {
    let _ = "foo".iter(); // { dg-error ".E0599." "" { target *-*-* } }
    let _ = "foo".foo(); // { dg-error ".E0599." "" { target *-*-* } }
    let _ = String::from("bar").iter(); // { dg-error ".E0599." "" { target *-*-* } }
    let _ = (&String::from("bar")).iter(); // { dg-error ".E0599." "" { target *-*-* } }
    let _ = 0.iter(); // { dg-error ".E0599." "" { target *-*-* } }
}

