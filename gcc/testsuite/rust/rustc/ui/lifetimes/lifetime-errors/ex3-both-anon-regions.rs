fn foo(x: &mut Vec<&u8>, y: &u8) {
    x.push(y);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() { }

