trait T {
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
    fn foo() {}
=======
    fn bar() {}
>>>>>>> branch
}

struct S;
impl T for S {}

fn main() {
    S::foo();
}

