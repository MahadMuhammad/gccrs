use std::fmt::Debug;

fn elided(x: &i32) -> impl Copy { x }
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn explicit<'a>(x: &'a i32) -> impl Copy { x }
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn elided2(x: &i32) -> impl Copy + 'static { x }
// { dg-error "" "" { target *-*-* } .-1 }

fn explicit2<'a>(x: &'a i32) -> impl Copy + 'static { x }
// { dg-error "" "" { target *-*-* } .-1 }

fn foo<'a>(x: &i32) -> impl Copy + 'a { x }
// { dg-error ".E0621." "" { target *-*-* } .-1 }

fn elided3(x: &i32) -> Box<dyn Debug> { Box::new(x) }
// { dg-error "" "" { target *-*-* } .-1 }

fn explicit3<'a>(x: &'a i32) -> Box<dyn Debug> { Box::new(x) }
// { dg-error "" "" { target *-*-* } .-1 }

fn elided4(x: &i32) -> Box<dyn Debug + 'static> { Box::new(x) }
// { dg-error "" "" { target *-*-* } .-1 }

fn explicit4<'a>(x: &'a i32) -> Box<dyn Debug + 'static> { Box::new(x) }
// { dg-error "" "" { target *-*-* } .-1 }

fn elided5(x: &i32) -> (Box<dyn Debug>, impl Debug) { (Box::new(x), x) }
// { dg-error "" "" { target *-*-* } .-1 }

trait LifetimeTrait<'a> {}
impl<'a> LifetimeTrait<'a> for &'a i32 {}

fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
// { dg-error "" "" { target *-*-* } .-1 }

// Tests that a closure type containing 'b cannot be returned from a type where
// only 'a was expected.
fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {
    move |_| println!("{}", y)
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn ty_param_wont_outlive_static<T:Debug>(x: T) -> impl Debug + 'static {
    x
// { dg-error ".E0310." "" { target *-*-* } .-1 }
}

fn main() {}

