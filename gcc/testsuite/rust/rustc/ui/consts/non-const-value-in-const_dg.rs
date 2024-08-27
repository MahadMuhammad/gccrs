fn main() {
    let x = 5;
    const Y: i32 = x; // { dg-error ".E0435." "" { target *-*-* } }

    let x = 5;
    let _ = [0; x]; // { dg-error ".E0435." "" { target *-*-* } }
}

