struct Foo {
    x: isize,
}

impl Foo {
    fn hi(&self) -> bool {
        true
    }
}

fn main() {
    if Foo { // { dg-error "" "" { target *-*-* } }
        x: 3
    }.hi() {
        println!("yo");
    }
    if let true = Foo { // { dg-error "" "" { target *-*-* } }
        x: 3
    }.hi() {
        println!("yo");
    }
}

