fn main() {
    let _ = [0; (&0 as *const i32) as usize]; // { dg-error "" "" { target *-*-* } }
}

