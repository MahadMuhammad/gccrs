fn foo()
where
    T: Send,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
    let s = "abc".to_string();
}

fn main() {}

