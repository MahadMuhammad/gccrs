fn main() {
    let fields = vec![1];
    let variant = vec![2];

    // should not suggest `*&variant.iter()`
    for (src, dest) in std::iter::zip(fields.iter(), &variant.iter()) {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
        eprintln!("{} {}", src, dest);
    }

    // don't suggest add `variant.iter().clone().clone()`
    for (src, dest) in std::iter::zip(fields.iter(), &variant.iter().clone()) {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
        eprintln!("{} {}", src, dest);
    }
}

