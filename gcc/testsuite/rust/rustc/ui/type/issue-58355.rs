#![crate_type = "lib"]

pub fn foo(callback: fn() -> dyn ToString) {
    let mut x: Option<Box<dyn Fn() -> dyn ToString>> = None;
    x = Some(Box::new(callback));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

