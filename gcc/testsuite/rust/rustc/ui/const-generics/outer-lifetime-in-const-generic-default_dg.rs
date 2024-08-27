struct Foo<
    'a,
    const N: usize = {
        let x: &'a ();
// { dg-error "" "" { target *-*-* } .-1 }
        3
    },
>(&'a ());

fn main() {}

