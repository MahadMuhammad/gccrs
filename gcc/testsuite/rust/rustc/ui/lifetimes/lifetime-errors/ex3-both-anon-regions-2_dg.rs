fn foo(&mut (ref mut v, w): &mut (&u8, &u8), x: &u8) {
    *v = x;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() { }

