trait Trait<'a>: 'a {
    type Type;
}

// if the `T: 'a` bound gets implied we would probably get ub here again
impl<'a, T> Trait<'a> for T {
// { dg-error ".E0309." "" { target *-*-* } .-1 }
// { dg-error ".E0309." "" { target *-*-* } .-2 }
    type Type = ();
}

fn f<'a, 'b>(s: &'b str, _: <&'b () as Trait<'a>>::Type) -> &'a str
where
    &'b (): Trait<'a>,
{
    s
}

fn main() {
    let x = String::from("Hello World!");
    let y = f(&x, ());
    drop(x); // { dg-error ".E0505." "" { target *-*-* } }
    println!("{}", y);
}

