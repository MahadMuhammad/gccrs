fn main() {
    let y = Some(0);
    if let Some(x) = y {
        x = 2; // { dg-error ".E0384." "" { target *-*-* } }
    }

    let mut arr = [1, 2, 3];
    let [x, ref xs_hold @ ..] = arr;
    x = 0; // { dg-error ".E0384." "" { target *-*-* } }
    eprintln!("{:?}", arr);
}

