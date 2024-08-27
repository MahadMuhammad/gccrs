fn function<T>(x: T, y: bool) -> T {
    x
}

struct S {}
impl S {
    fn method<T>(&self, x: T) -> T {
        x
    }
}

fn wrong_arg_type(x: u32) -> u32 {
    x
}

fn main() {
    // Should not trigger.
    let x = wrong_arg_type(0u16); // { dg-error ".E0308." "" { target *-*-* } }
    let x: u16 = function(0, 0u8); // { dg-error ".E0308." "" { target *-*-* } }

    // Should trigger exactly once for the first argument.
    let x: u16 = function(0u32, 0u8); // { dg-error ".E0308." "" { target *-*-* } }

    // Should trigger.
    let x: u16 = function(0u32, true); // { dg-error ".E0308." "" { target *-*-* } }
    let x: u16 = (S {}).method(0u32); // { dg-error ".E0308." "" { target *-*-* } }
    function(0u32, 8u8) // { dg-error ".E0308." "" { target *-*-* } }
}

