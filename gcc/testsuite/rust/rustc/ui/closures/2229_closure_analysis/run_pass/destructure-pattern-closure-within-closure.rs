// { dg-additional-options "-frust-edition=2021" }
//@ check-pass
#![warn(unused)]

fn main() {
    let t = (String::from("Hello"), String::from("World"));
    let g = (String::from("Mr"), String::from("Goose"));

    let a = || {
        let (_, g2) = g;
// { dg-warning "" "" { target *-*-* } .-1 }
        let c = ||  {
            let (_, t2) = t;
// { dg-warning "" "" { target *-*-* } .-1 }
        };

        c();
    };

    a();
}

