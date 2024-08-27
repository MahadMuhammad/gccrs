//@ run-rustfix

fn take_str_maybe(_: Option<&str>) { }
fn main() {
    let string = String::from("Hello, world");

    let option: Option<String> = Some(string.clone());
    take_str_maybe(option);
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let option_ref = Some(&string);
    take_str_maybe(option_ref);
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let option_ref_ref = option_ref.as_ref();
    take_str_maybe(option_ref_ref);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

