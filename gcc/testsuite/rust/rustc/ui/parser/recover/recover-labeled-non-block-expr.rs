//@ run-rustfix
fn main() {
    let _ = 'label: 1 + 1; // { dg-error "" "" { target *-*-* } }

    'label: match () { () => {}, }; // { dg-error "" "" { target *-*-* } }
    'label: match () { () => break 'label, }; // { dg-error "" "" { target *-*-* } }
    #[allow(unused_labels)]
    'label: match () { () => 'lp: loop { break 'lp 0 }, }; // { dg-error "" "" { target *-*-* } }

    let x = 1;
    let _i = 'label: match x { // { dg-error "" "" { target *-*-* } }
        0 => 42,
        1 if false => break 'label 17,
        1 => {
            if true {
                break 'label 13
            } else {
                break 'label 0;
            }
        }
        _ => 1,
    };

    let other = 3;
    let _val = 'label: (1, if other == 3 { break 'label (2, 3) } else { other }); // { dg-error "" "" { target *-*-* } }
}

