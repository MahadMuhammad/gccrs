fn foo(x: bool) -> i32 {
    match x {
        x: i32 => x, // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        true => 42.,
        false => 0.333,
    }
}

fn main() {
    match foo(true) {
        42: i32 => (), // { dg-error "" "" { target *-*-* } }
        _: f64 => (), // { dg-error "" "" { target *-*-* } }
        x: i32 => (), // { dg-error "" "" { target *-*-* } }
    }
}

