// ported from <https://github.com/rust-lang/rust/blob/master/tests/ui/associated-types/issue-22037.rs>
trait A {
    type Output;
    fn a(&self) -> <Self as A>::X;
    // { dg-error "cannot find associated type .X. in trait .A. .E0576." ""  { target *-*-* } }
}

impl A for u32 {
    type Output = u32;
    fn a(&self) -> u32 {
        0
    }
}

fn main() {
    let a: u32 = 0;
    let b: u32 = a.a();
}