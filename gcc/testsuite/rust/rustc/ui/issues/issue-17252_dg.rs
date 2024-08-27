const FOO: usize = FOO; // { dg-error ".E0391." "" { target *-*-* } }

fn main() {
    let _x: [u8; FOO]; // caused stack overflow prior to fix
    let _y: usize = 1 + {
        const BAR: usize = BAR;
// { dg-error ".E0391." "" { target *-*-* } .-1 }
        let _z: [u8; BAR]; // caused stack overflow prior to fix
        1
    };
}

