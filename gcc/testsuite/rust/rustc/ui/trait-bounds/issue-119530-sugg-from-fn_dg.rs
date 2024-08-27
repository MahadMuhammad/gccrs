fn foo() -> String { String::new() }

fn main() {
    let string_arr = [foo(); 64]; // { dg-error ".E0277." "" { target *-*-* } }
}

