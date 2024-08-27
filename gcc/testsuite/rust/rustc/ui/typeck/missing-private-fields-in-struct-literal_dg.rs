pub mod m {
    pub struct S {
        pub visible: bool,
        a: (),
        b: (),
        c: (),
        d: (),
        e: (),
    }
}

fn main() {
    let _ = m::S { // { dg-error "" "" { target *-*-* } }
        visible: true,
        a: (),
        b: (),
    };
}

