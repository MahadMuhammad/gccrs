#[attribute]
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
fn foo() {}
=======
fn bar() {}
>>>>>>> branch

fn main() {
    foo();
}

