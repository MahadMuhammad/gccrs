enum E {
    Foo {
<<<<<<< HEAD // { dg-error "" "" { target *-*-* } }
        x: u8,
|||||||
        z: (),
=======
        y: i8,
>>>>>>> branch
    }
}

