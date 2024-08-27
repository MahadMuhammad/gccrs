// Regression test for one part of issue #105306.

fn main() {
    let _ = Option::<[u8]>::None;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

