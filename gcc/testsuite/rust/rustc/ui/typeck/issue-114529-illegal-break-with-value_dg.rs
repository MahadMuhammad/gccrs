// Regression test for issue #114529
// Tests that we do not ICE during const eval for a
// break-with-value in contexts where it is illegal

#[allow(while_true)]
fn main() {
    [(); {
        while true {
            break 9; // { dg-error ".E0571." "" { target *-*-* } }
        };
        51
    }];

    [(); {
        while let Some(v) = Some(9) {
            break v; // { dg-error ".E0571." "" { target *-*-* } }
        };
        51
    }];

    while true {
        break (|| { // { dg-error ".E0571." "" { target *-*-* } }
            let local = 9;
        });
    }
}

