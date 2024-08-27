//@ check-pass
// Tests that we properly lint at 'paren' expressions

fn foo() -> Result<(), String>  {
    (try!(Ok::<u8, String>(1))); // { dg-warning "" "" { target *-*-* } }
    Ok(())
}

fn main() {}

