type Real = double;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }

fn main() {
    let x: Real = 3.5;
    let y: long = 74802374902374923;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
    let v1: Boolean = true;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
    let v2: Bool = true;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
// { help ".E0412." "" { target *-*-* } .-3 }
}

fn z(a: boolean) {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
}

fn a() -> byte {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
    3
}

struct Data { // { help "" "" { target *-*-* } }
    width: float,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
    depth: Option<int>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
}

trait Stuff {}
impl Stuff for short {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }

