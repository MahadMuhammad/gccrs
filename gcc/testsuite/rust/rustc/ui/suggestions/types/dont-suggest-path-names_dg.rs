// This is a regression test for #123630
//
// Prior to #123703 this was resulting in compiler suggesting add a type signature
// for `lit` containing path to a file containing `Select` - something obviously invalid.

struct Select<F, I>(F, I);
fn select<F, I>(filter: F) -> Select<F, I> {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn parser1() {
    let lit = select(|x| match x {
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        _ => (),
    });
}

fn main() {}

