struct S(());

fn main() {
    let array = [S(())];

    match array {
        [()] => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        _ => {}
    }
}

