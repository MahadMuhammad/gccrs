use std::convert::identity;

fn test<'a: 'a>(n: bool) -> impl Sized + 'a {
// { dg-error "" "" { target *-*-* } .-1 }
    let true = n else { loop {} };
    let _ = || {
        let _ = identity::<&'a ()>(test(false));
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    };
    loop {}
}

fn main() {}

