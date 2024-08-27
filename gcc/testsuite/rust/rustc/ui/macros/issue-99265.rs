//@ check-pass
//@ run-rustfix

fn main() {
    println!("{b} {}", a=1, b=2);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("{} {} {} {} {}", 0, a=1, b=2, c=3, d=4);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
// { help "" "" { target *-*-* } .-7 }
// { help "" "" { target *-*-* } .-8 }

    println!("Hello {:1$}!", "x", width = 5);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {:1$.2$}!", f = 0.02f32, width = 5, precision = 2);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }

    println!("Hello {0:1$.2$}!", f = 0.02f32, width = 5, precision = 2);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }

    println!(
        "{}, Hello {1:2$.3$} {4:5$.6$}! {1}",
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
// { help "" "" { target *-*-* } .-7 }
        1,
        f = 0.02f32,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        width = 5,
// { dg-warning "" "" { target *-*-* } .-1 }
        precision = 2,
// { dg-warning "" "" { target *-*-* } .-1 }
        g = 0.02f32,
// { dg-warning "" "" { target *-*-* } .-1 }
        width2 = 5,
// { dg-warning "" "" { target *-*-* } .-1 }
        precision2 = 2
// { dg-warning "" "" { target *-*-* } .-1 }
    );

    println!("Hello {:0.1}!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {0:0.1}!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {f:width$.precision$}!", f = 0.02f32, width = 5, precision = 2);

    let width = 5;
    let precision = 2;
    println!("Hello {f:width$.precision$}!", f = 0.02f32);

    let val = 5;
    println!("{:0$}", v = val);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
    println!("{0:0$}", v = val);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
    println!("{:0$.0$}", v = val);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
    println!("{0:0$.0$}", v = val);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }

    println!("{} {a} {0}", a = 1);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

    println!("aaaaaaaaaaaaaaa\
                {:1$.2$}",
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
             a = 1.0, b = 1, c = 2,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    );

    println!("aaaaaaaaaaaaaaa\
                {0:1$.2$}",
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
             a = 1.0, b = 1, c = 2,
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    );

    println!("{{{:1$.2$}}}", x = 1.0, width = 3, precision = 2);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
}

