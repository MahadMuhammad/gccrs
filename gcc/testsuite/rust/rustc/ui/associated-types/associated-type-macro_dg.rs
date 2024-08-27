fn main() {
    #[cfg(FALSE)]
    <() as module>::mac!(); // { dg-error "" "" { target *-*-* } }
}

