// Don't suggest double quotes when encountering an expr of type `char` where a `&str`
// is expected if the expr is not a char literal.
// issue: rust-lang/rust#125595

fn main() {
    let _: &str = ('a'); // { dg-error ".E0308." "" { target *-*-* } }
    let token = || 'a';
    let _: &str = token(); // { dg-error ".E0308." "" { target *-*-* } }
}

