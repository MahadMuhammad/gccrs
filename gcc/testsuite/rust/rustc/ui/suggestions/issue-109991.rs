struct S {
    a: usize,
    b: usize,
}

fn main() {
    let a: usize;
    let b: usize;
    let c: usize;

    (c) = (&123); // { dg-error ".E0308." "" { target *-*-* } }
    (a, b) = (123, &mut 123); // { dg-error ".E0308." "" { target *-*-* } }

    let x: String;
    (x,) = (1,); // { dg-error ".E0308." "" { target *-*-* } }

    let x: i32;
    [x] = [&1]; // { dg-error ".E0308." "" { target *-*-* } }

    let x: &i32;
    [x] = [1]; // { dg-error ".E0308." "" { target *-*-* } }

    let x = (1, &mut 2);
    (a, b) = x; // { dg-error ".E0308." "" { target *-*-* } }

    S { a, b } = S { a: 1, b: &mut 2 }; // { dg-error ".E0308." "" { target *-*-* } }
}

