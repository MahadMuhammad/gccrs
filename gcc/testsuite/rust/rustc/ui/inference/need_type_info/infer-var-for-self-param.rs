// Regression test for #113610 where we ICEd when trying to print
// inference variables created by instantiating the self type parameter.

fn main() {
    let _ = (Default::default(),);
// { dg-error ".E0790." "" { target *-*-* } .-1 }
}

