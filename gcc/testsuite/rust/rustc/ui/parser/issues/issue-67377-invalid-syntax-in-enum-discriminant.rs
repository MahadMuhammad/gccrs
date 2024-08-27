mod a {
    use std::marker::PhantomData;

    enum Bug {
        V = [PhantomData; { [ () ].len() ].len() as isize,
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

mod b {
    enum Bug {
        V = [Vec::new; { [].len()  ].len() as isize,
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

mod c {
    enum Bug {
        V = [Vec::new; { [0].len() ].len() as isize,
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {} // { dg-error "" "" { target *-*-* } }

