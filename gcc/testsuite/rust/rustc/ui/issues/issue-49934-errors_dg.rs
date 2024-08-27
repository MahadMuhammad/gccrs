fn foo<#[derive(Debug)] T>() { // { dg-error "" "" { target *-*-* } }
    match 0 {
        #[derive(Debug)] // { dg-error "" "" { target *-*-* } }
        _ => (),
    }
}

fn main() {}

