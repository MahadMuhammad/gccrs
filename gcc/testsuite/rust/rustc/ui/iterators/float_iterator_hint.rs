// #106728

fn main() {
    for i in 0.2 {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
// { dg-note ".E0277." "" { target *-*-* } .-5 }
// { dg-note ".E0277." "" { target *-*-* } .-6 }
// { dg-note ".E0277." "" { target *-*-* } .-7 }
// { dg-note ".E0277." "" { target *-*-* } .-8 }
        println!();
    }
}

