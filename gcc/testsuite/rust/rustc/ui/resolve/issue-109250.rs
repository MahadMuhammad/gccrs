fn main() {       // { help "" "" { target *-*-* } }
    HashMap::new; // { dg-error ".E0433." "" { target *-*-* } }
}

