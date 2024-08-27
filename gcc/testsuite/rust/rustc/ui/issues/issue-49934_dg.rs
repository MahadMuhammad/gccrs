#![feature(stmt_expr_attributes)]

fn main() {
    // fold_stmt (Item)
    #[allow(dead_code)]
    #[derive(Debug)] // should not warn
    struct Foo;

    // fold_stmt (Mac)
    #[derive(Debug)] // { dg-error ".E0774." "" { target *-*-* } }
    println!("Hello, world!");

    // fold_stmt (Semi)
    #[derive(Debug)] // { dg-error ".E0774." "" { target *-*-* } }
    "Hello, world!";

    // fold_stmt (Local)
    #[derive(Debug)] // { dg-error ".E0774." "" { target *-*-* } }
    let _ = "Hello, world!";

    // visit_expr
    let _ = #[derive(Debug)] "Hello, world!";
// { dg-error ".E0774." "" { target *-*-* } .-1 }

    let _ = [
        // filter_map_expr
        #[derive(Debug)] // { dg-error ".E0774." "" { target *-*-* } }
        "Hello, world!",
    ];
}

