//@ check-pass
//
// Ensures that we properly lint
// a removed 'expression' resulting from a macro
// in trailing expression position

macro_rules! expand_it {
    () => {
        #[cfg(FALSE)] 25; // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
    }
}

fn main() {
    expand_it!()
}

