struct Foo;

fn main() {
    let a: Result<(), Foo> = Ok(());
    a.unwrap();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

