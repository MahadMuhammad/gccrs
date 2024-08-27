fn main() {
    println!(
        r#"
    \"\'}､"# // { dg-error "" "" { target *-*-* } }
    );
}

