trait T {
    fn foo(
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
        x: u8,
=======
        x: i8,
>>>>>>> branch
    ) {}
}

struct S;
impl T for S {}

fn main() {
    S::foo(42);
}

