fn main() {
    let _my_usize = const {
        let x: bool;
        while x {} // { dg-error ".E0381." "" { target *-*-* } }
    };
}

