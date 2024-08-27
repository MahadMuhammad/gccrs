// regression test for #108683
// { dg-additional-options "-frust-edition=2021" }

enum Refutable {
    A,
    B,
}

fn example(v1: u32, v2: [u32; 4], v3: Refutable) {
    const PAT: u32 = 0;
    let v4 = &v2[..];
    || {
        let 0 = v1; // { dg-error ".E0005." "" { target *-*-* } }
        let (0 | 1) = v1; // { dg-error ".E0005." "" { target *-*-* } }
        let 1.. = v1; // { dg-error ".E0005." "" { target *-*-* } }
        let [0, 0, 0, 0] = v2; // { dg-error ".E0005." "" { target *-*-* } }
        let [0] = v4; // { dg-error ".E0005." "" { target *-*-* } }
        let Refutable::A = v3; // { dg-error ".E0005." "" { target *-*-* } }
        let PAT = v1; // { dg-error ".E0005." "" { target *-*-* } }
    };
}

fn main() {}

