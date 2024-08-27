//@ revisions: edition2015 edition2021

mod config; // { dg-error "" "" { target *-*-* } }

fn main() {
    match &args.cmd { // { dg-error "" "" { target *-*-* } }
        crate::config => {} // { dg-error "" "" { target *-*-* } }
    }

    println!(args.ctx.compiler.display());
// { dg-error "" "" { target *-*-* } .-1 }
}

