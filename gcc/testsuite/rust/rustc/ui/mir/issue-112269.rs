pub fn main() {
    const x: i32 = 4;
    let x: i32 = 3;
// { dg-error ".E0005." "" { target *-*-* } .-1 }

    const y: i32 = 3;
    let y = 4;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
}

