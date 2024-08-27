fn main() {
    let a = std::collections::HashMap::<String,String>::new();
    let s = "hello";
    let _b = a[
        &s // { dg-error ".E0277." "" { target *-*-* } }
    ];
}

