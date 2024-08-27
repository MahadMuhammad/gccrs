#![deny(unconditional_recursion)]

#![allow(dead_code)]
fn foo() { // { dg-error "" "" { target *-*-* } }
    foo();
}

fn bar() {
    if true {
        bar()
    }
}

fn baz() { // { dg-error "" "" { target *-*-* } }
    if true {
        baz()
    } else {
        baz()
    }
}

fn qux() {
    loop {}
}

fn quz() -> bool { // { dg-error "" "" { target *-*-* } }
    if true {
        while quz() {}
        true
    } else {
        loop { quz(); }
    }
}

// Trait method calls.
trait Foo {
    fn bar(&self) { // { dg-error "" "" { target *-*-* } }
        self.bar()
    }
}

impl Foo for Box<dyn Foo + 'static> {
    fn bar(&self) { // { dg-error "" "" { target *-*-* } }
        loop {
            self.bar()
        }
    }
}

// Trait method call with integer fallback after method resolution.
impl Foo for i32 {
    fn bar(&self) { // { dg-error "" "" { target *-*-* } }
        0.bar()
    }
}

impl Foo for u32 {
    fn bar(&self) {
        0.bar()
    }
}

// Trait method calls via paths.
trait Foo2 {
    fn bar(&self) { // { dg-error "" "" { target *-*-* } }
        Foo2::bar(self)
    }
}

impl Foo2 for Box<dyn Foo2 + 'static> {
    fn bar(&self) { // { dg-error "" "" { target *-*-* } }
        loop {
            Foo2::bar(self)
        }
    }
}

struct Baz;
impl Baz {
    // Inherent method call.
    fn qux(&self) { // { dg-error "" "" { target *-*-* } }
        self.qux();
    }

    // Inherent method call via path.
    fn as_ref(&self) -> &Self { // { dg-error "" "" { target *-*-* } }
        Baz::as_ref(self)
    }
}

// Trait method calls to impls via paths.
impl Default for Baz {
    fn default() -> Baz { // { dg-error "" "" { target *-*-* } }
        let x = Default::default();
        x
    }
}

// Overloaded operators.
impl std::ops::Deref for Baz {
    type Target = ();
    fn deref(&self) -> &() { // { dg-error "" "" { target *-*-* } }
        &**self
    }
}

impl std::ops::Index<usize> for Baz {
    type Output = Baz;
    fn index(&self, x: usize) -> &Baz { // { dg-error "" "" { target *-*-* } }
        &self[x]
    }
}

// Overloaded autoderef.
struct Quux;
impl std::ops::Deref for Quux {
    type Target = Baz;
    fn deref(&self) -> &Baz { // { dg-error "" "" { target *-*-* } }
        self.as_ref()
    }
}

fn all_fine() {
    let _f = all_fine;
}

// issue 26333
trait Bar {
    fn method<T: Bar>(&self, x: &T) {
        x.method(x)
    }
}

// Do not trigger on functions that may diverge instead of self-recursing (#54444)

pub fn loops(x: bool) {
    if x {
        loops(x);
    } else {
        loop {}
    }
}

pub fn panics(x: bool) {
    if x {
        panics(!x);
    } else {
        panic!("panics");
    }
}

pub fn unreachable1() {
    panic!();
    unreachable1(); // WARN unreachable statement
}

pub fn unreachable2() {
    loop {}
    unreachable2(); // WARN unreachable statement
}

pub fn drop_and_replace(mut a: Option<String>) { // { dg-error "" "" { target *-*-* } }
    a = None;
    drop_and_replace(a);
}

// Calls are assumed to return normally.
pub fn call() -> String { // { dg-error "" "" { target *-*-* } }
    let s = String::new();
    call();
    s
}

// Arithmetic operations are assumed not to overflow.
pub fn overflow_check(a: i32, b: i32) { // { dg-error "" "" { target *-*-* } }
    let _ = a + b;
    overflow_check(a, b);
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Default for Point {
    fn default() -> Self { // { dg-error "" "" { target *-*-* } }
        Point {
            x: Default::default(),
            ..Default::default()
        }
    }
}

fn main() {}

