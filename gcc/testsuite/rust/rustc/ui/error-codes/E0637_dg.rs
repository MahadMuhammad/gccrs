fn underscore_lifetime<'_>(str1: &'_ str, str2: &'_ str) -> &'_ str {
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn and_without_explicit_lifetime<T>()
where
    T: Into<&u32>, // { dg-error ".E0637." "" { target *-*-* } }
{
}

fn main() {}

