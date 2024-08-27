struct Argument;
struct Return;

fn function(_: Argument) -> Return { todo!() }

trait Trait {}
impl Trait for fn(Argument) -> Return {}

fn takes(_: impl Trait) {}

fn main() {
    takes(function);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    takes(|_: Argument| -> Return { todo!() });
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

