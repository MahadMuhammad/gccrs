fn test() {
    let x = match **x { // { dg-error ".E0425." "" { target *-*-* } }
        Some(&a) if { panic!() } => {}
    };
    let mut p = &x;

    {
        let mut closure = expect_sig(|p, y| *p = y);
        closure(&mut p, &y); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }

    deref(p); // { dg-error ".E0423." "" { target *-*-* } }
}

fn expect_sig<F>(f: F) -> F
where
    F: FnMut(&mut &i32, &i32),
{
    f
}

fn main() {}

