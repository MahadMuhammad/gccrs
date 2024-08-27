const fn concat_strs<const A: &'static str>() -> &'static str {
// { dg-error "" "" { target *-*-* } .-1 }
    struct Inner<const A: &'static str>;
// { dg-error "" "" { target *-*-* } .-1 }
    Inner::concat_strs::<"a">::A
// { dg-error ".E0223." "" { target *-*-* } .-1 }
}

pub fn main() {}

