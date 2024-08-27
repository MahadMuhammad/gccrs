// Issue #94176: wrong span for the error message of a mismatched type error,
// if the function uses a `let else` construct.


pub fn test(a: Option<u32>) -> Option<u32> { // { dg-error ".E0308." "" { target *-*-* } }
    let Some(_) = a else { return None; };
    println!("Foo");
}

fn main() {}

