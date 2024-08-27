//@ compile-flags: --test

#[test] // { dg-error "" "" { target *-*-* } }
mod test {}

#[test] // { dg-error "" "" { target *-*-* } }
mod loooooooooooooong_teeeeeeeeeest {
    /*
    this is a comment
    this comment goes on for a very long time
    this is to pad out the span for this module for a long time
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
    labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
    laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
    voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
    non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    */
}

#[test] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[test] // { dg-error "" "" { target *-*-* } }
trait Foo {}

#[test] // { dg-error "" "" { target *-*-* } }
impl Foo for i32 {}

#[test] // { dg-error "" "" { target *-*-* } }
const FOO: i32 = -1_i32;

#[test] // { dg-error "" "" { target *-*-* } }
static BAR: u64 = 10_000_u64;

#[test] // { dg-error "" "" { target *-*-* } }
enum MyUnit {
    Unit,
}

#[test] // { dg-error "" "" { target *-*-* } }
struct NewI32(i32);

#[test] // { dg-error "" "" { target *-*-* } }
union Spooky {
    x: i32,
    y: u32,
}

#[repr(C, align(64))]
#[test] // { dg-error "" "" { target *-*-* } }
#[derive(Copy, Clone, Debug)]
struct MoreAttrs {
    a: i32,
    b: u64,
}

macro_rules! foo {
    () => {};
}

#[test] // { dg-warning "" "" { target *-*-* } }
foo!();

// make sure it doesn't erroneously trigger on a real test
#[test]
fn real_test() {
    assert_eq!(42_i32, 42_i32);
}

// make sure it works with cfg test
#[cfg(test)]
mod real_tests {
    #[cfg(test)]
    fn foo() {}

    #[test]
    fn bar() {
        foo();
    }
}

