// ICE size_and_align_of::<[closure@test.rs:15:5: 17:7]> not supported #88212
// issue: rust-lang/rust#88212
#![feature(unsized_locals)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Example {}
struct Foo();

impl Example for Foo {}

fn example() -> Box<dyn Example> {
    Box::new(Foo())
}

fn main() {
    let x: dyn Example = *example();
    (move || {
        let _y = x;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    })();
}

