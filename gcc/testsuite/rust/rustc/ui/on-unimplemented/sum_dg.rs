// <https://github.com/rust-lang/rust/issues/105184>

fn main() {
    vec![(), ()].iter().sum::<i32>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    vec![(), ()].iter().product::<i32>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

