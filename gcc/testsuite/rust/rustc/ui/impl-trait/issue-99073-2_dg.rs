use std::fmt::Display;

fn main() {
    test("hi", true);
}

fn test<T: Display>(t: T, recurse: bool) -> impl Display {
    let f = || {
        let i: u32 = test::<i32>(-1, false);
// { dg-error ".E0792." "" { target *-*-* } .-1 }
// { dg-error ".E0792." "" { target *-*-* } .-2 }
        println!("{i}");
    };
    if recurse {
        f();
    }
    t
}

