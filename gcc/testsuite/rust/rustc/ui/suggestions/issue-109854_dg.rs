fn generate_setter() {
    String::with_capacity(
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    generate_setter,
    r#"
pub(crate) struct Person<T: Clone> {}
"#,
     r#""#,
    );
}

fn main() {}

