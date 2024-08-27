// Make sure we don't ICE when a foreign fn doesn't implement `Fn` due to arg mismatch.

unsafe extern "Rust" {
    pub safe fn foo();
    pub safe fn bar(x: u32);
}

fn test(_: impl Fn(i32)) {}

fn main() {
    test(foo); // { dg-error ".E0593." "" { target *-*-* } }
    test(bar); // { dg-error ".E0631." "" { target *-*-* } }
}

