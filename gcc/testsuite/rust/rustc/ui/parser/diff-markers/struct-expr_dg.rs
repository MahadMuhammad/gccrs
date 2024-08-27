struct S {
    x: u8,
}
fn main() {
    let _ = S {
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
        x: 42,
=======
        x: 0,
>>>>>>> branch
    }
}

