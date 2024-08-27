//@ check-pass

fn main() {
    let x = 1 else { return }; // { dg-warning "" "" { target *-*-* } }

    // Multiline else blocks should not get printed
    let x = 1 else { // { dg-warning "" "" { target *-*-* } }
        eprintln!("problem case encountered");
        return
    };
}

