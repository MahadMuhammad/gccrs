trait X {
    type Y<'a>;
}

fn f1<'a>(arg : Box<dyn X<Y = B = &'a ()>>) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

