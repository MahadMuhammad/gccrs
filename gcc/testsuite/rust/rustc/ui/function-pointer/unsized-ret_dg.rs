#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]

fn foo<F: Fn<T>, T:std::marker::Tuple>(f: Option<F>, t: T) {
    let y = (f.unwrap()).call(t);
}

fn main() {
    foo::<fn() -> str, _>(None, ());
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    foo::<for<'a> fn(&'a ()) -> (dyn std::fmt::Display + 'a), _>(None, (&(),));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

