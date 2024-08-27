fn main() {
    let _ = 0: i32; // { dg-error "" "" { target *-*-* } }
    let _ = 0: i32; // (error only emitted once)
}

