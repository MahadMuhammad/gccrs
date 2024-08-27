//@ run-rustfix
fn main() {
    {
        break (); // { dg-error ".E0268." "" { target *-*-* } }
    }
    {
        {
            break (); // { dg-error ".E0268." "" { target *-*-* } }
        }
    }
}

