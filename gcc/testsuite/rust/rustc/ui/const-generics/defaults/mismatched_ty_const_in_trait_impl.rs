trait Trait {
    fn foo<U>() {}
}
impl Trait for () {
    fn foo<const M: u64>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

trait Other {
    fn bar<const M: u8>() {}
}
impl Other for () {
    fn bar<T>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

trait Uwu {
    fn baz<const N: u32>() {}
}
impl Uwu for () {
    fn baz<const N: i32>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

trait Aaaaaa {
    fn bbbb<const N: u32, T>() {}
}
impl Aaaaaa for () {
    fn bbbb<T, const N: u32>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

trait Names {
    fn abcd<T, const N: u32>() {}
}
impl Names for () {
    fn abcd<const N: u32, T>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

fn main() {}

