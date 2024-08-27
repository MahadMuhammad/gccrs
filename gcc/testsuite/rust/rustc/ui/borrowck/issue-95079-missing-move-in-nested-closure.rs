//@ run-rustfix
#![allow(dead_code, path_statements)]
fn foo1(s: &str) -> impl Iterator<Item = String> + '_ {
    None.into_iter()
        .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn foo2(s: &str) -> impl Sized + '_ {
    move |()| s.chars().map(|c| format!("{}{}", c, s))
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

pub struct X;
pub fn foo3<'a>(
    bar: &'a X,
) -> impl Iterator<Item = ()> + 'a {
    Some(()).iter().flat_map(move |()| {
        Some(()).iter().map(|()| { bar; }) // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    })
}

fn main() {}

