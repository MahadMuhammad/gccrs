#![allow(dead_code)]

fn bar<'a>(_: std::fmt::Arguments<'a>) {}
fn main() {
    let x = format_args!("a {} {} {}.", 1, format_args!("b{}!", 2), 3);
// { dg-error ".E0716." "" { target *-*-* } .-1 }

    bar(x);

    let foo = format_args!("{}", "hi");
// { dg-error ".E0716." "" { target *-*-* } .-1 }
    bar(foo);

    let foo = format_args!("hi"); // no placeholder in arguments, so no error
    bar(foo);
}

