// This checks that function pointer signatures that are referenced mutably
// but contain a &mut T parameter still fail in a constant context: see issue #114994.
//
//@ check-fail

const fn use_mut_const_fn(_f: &mut fn(&mut String)) { // { dg-error ".E0658." "" { target *-*-* } }
    ()
}

const fn use_mut_const_tuple_fn(_f: (fn(), &mut u32)) { // { dg-error ".E0658." "" { target *-*-* } }

}

fn main() {}

