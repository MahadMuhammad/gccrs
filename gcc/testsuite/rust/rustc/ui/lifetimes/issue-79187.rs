fn thing(x: impl FnOnce(&u32)) {}

fn main() {
    let f = |_| ();
    thing(f);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

