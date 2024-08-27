// check-compile

struct DisplayOnly;

impl std::fmt::Display for DisplayOnly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

fn main() {
    let x = Some(1);
    println!("{x:?} {x} {x:?}");
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    println!("{x:?} {x} {x:?}", x = Some(1));
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let x = DisplayOnly;
    println!("{x} {x:?} {x}");
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    println!("{x} {x:?} {x}", x = DisplayOnly);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

