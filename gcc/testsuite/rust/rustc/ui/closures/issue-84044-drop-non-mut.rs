// #84044: This used to ICE.

fn main() {
    let f = || {};
    drop(&mut f); // { dg-error ".E0596." "" { target *-*-* } }
}

