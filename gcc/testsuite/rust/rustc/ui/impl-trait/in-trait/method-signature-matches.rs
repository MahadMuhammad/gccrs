// { dg-additional-options "-frust-edition= 2021" }
//@ revisions: mismatch mismatch_async too_many too_few lt

#[cfg(mismatch)]
trait Uwu {
    fn owo(x: ()) -> impl Sized;
}

#[cfg(mismatch)]
impl Uwu for () {
    fn owo(_: u8) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(mismatch_async)]
trait AsyncUwu {
    async fn owo(x: ()) {}
}

#[cfg(mismatch_async)]
impl AsyncUwu for () {
    async fn owo(_: u8) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(too_many)]
trait TooMuch {
    fn calm_down_please() -> impl Sized;
}

#[cfg(too_many)]
impl TooMuch for () {
    fn calm_down_please(_: (), _: (), _: ()) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(too_few)]
trait TooLittle {
    fn come_on_a_little_more_effort(_: (), _: (), _: ()) -> impl Sized;
}

#[cfg(too_few)]
impl TooLittle for () {
    fn come_on_a_little_more_effort() {}
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(lt)]
trait Lifetimes {
    fn early<'early, T>(x: &'early T) -> impl Sized;
}

#[cfg(lt)]
impl Lifetimes for () {
    fn early<'late, T>(_: &'late ()) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

