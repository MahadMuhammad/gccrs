// regression test for #109298
// { dg-additional-options "-frust-edition= 2021" }

pub fn subslice_array(x: [u8; 3]) {
    let f = || {
        let [_x @ ..] = x;
        let [ref y, ref mut z @ ..] = x; // { dg-error ".E0596." "" { target *-*-* } }
    };

    f(); // { dg-error ".E0596." "" { target *-*-* } }
}

fn main() {}

