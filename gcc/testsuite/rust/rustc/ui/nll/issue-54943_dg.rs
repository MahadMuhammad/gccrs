fn foo<T: 'static>() { }

fn boo<'a>() {
    return;

    let x = foo::<&'a u32>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

