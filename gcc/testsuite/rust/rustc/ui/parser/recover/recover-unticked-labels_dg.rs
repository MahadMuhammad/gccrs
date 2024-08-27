//@ run-rustfix

fn main() {
    'label: loop { break label };    // { dg-error ".E0425." "" { target *-*-* } }
    'label: loop { break label 0 };  // { dg-error "" "" { target *-*-* } }
    'label: loop { continue label }; // { dg-error "" "" { target *-*-* } }
}

