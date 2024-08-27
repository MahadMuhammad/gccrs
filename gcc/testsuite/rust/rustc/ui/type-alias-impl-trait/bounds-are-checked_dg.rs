// Make sure that we check that impl trait types implement the traits that they
// claim to.

#![feature(type_alias_impl_trait)]

type X<'a> = impl Into<&'static str> + From<&'a str>;

fn f<'a: 'static>(t: &'a str) -> X<'a> {
    t
// { dg-error ".E0792." "" { target *-*-* } .-1 }
}

fn extend_lt<'a>(o: &'a str) -> &'static str {
    X::<'_>::from(o).into()
}

fn main() {
    let r = {
        let s = "abcdef".to_string();
        extend_lt(&s)
    };
    println!("{}", r);
}

