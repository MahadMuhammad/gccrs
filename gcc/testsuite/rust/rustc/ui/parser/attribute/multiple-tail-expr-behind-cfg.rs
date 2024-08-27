#![feature(stmt_expr_attributes)]

fn foo() -> String {
    #[cfg(FALSE)]
    [1, 2, 3].iter().map(|c| c.to_string()).collect::<String>() // { dg-error "" "" { target *-*-* } }
    #[cfg(not(FALSE))]
    String::new()
}

fn bar() -> String {
    #[attr]
    [1, 2, 3].iter().map(|c| c.to_string()).collect::<String>() // { dg-error "" "" { target *-*-* } }
    #[attr] // { dg-error "" "" { target *-*-* } }
    String::new()
}

fn main() {
    println!("{}", foo());
}

