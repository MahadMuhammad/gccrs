fn never() -> ! {
    loop {}
}

fn bar(a: bool) {
    match a {
        true => 1,
        false => {
            never() // { dg-error ".E0308." "" { target *-*-* } }
        }
    }
}
fn main() {
    bar(true);
    bar(false);
}

