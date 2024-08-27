//@ run-pass
//@ pretty-expanded FIXME #23616

trait Str { fn foo(&self) {} } // { dg-warning "" "" { target *-*-* } }
impl Str for str {}
impl<'a, S: ?Sized> Str for &'a S where S: Str {}

fn main() {
    let _: &dyn Str = &"x";
}

