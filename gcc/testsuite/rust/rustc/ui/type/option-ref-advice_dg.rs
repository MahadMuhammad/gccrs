// Regression test for https://github.com/rust-lang/rust/issues/100605

fn takes_option(_arg: Option<&String>) {}

fn main() {
    takes_option(&None); // { dg-error ".E0308." "" { target *-*-* } }

    let x = String::from("x");
    let res = Some(x);
    takes_option(&res); // { dg-error ".E0308." "" { target *-*-* } }
}

