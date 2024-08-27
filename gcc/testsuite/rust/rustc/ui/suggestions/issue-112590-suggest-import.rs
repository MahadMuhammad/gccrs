pub struct S;

impl fmt::Debug for S { // { dg-error ".E0433." "" { target *-*-* } }
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { // { dg-error ".E0433." "" { target *-*-* } }
// { dg-error ".E0433." "" { target *-*-* } .-1 }
        Ok(())
    }
}

fn main() { }

