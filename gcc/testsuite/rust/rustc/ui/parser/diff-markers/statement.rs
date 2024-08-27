trait T {
    fn foo() {}
    fn bar() {}
}

struct S;
impl T for S {}

fn main() {
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
    S::foo();
=======
    S::bar();
>>>>>>> branch
}

