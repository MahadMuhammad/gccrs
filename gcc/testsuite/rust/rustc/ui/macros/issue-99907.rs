//@ check-pass
//@ run-rustfix

fn main() {
    println!("Hello {:.1}!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {:1.1}!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {}!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello { }!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    println!("Hello {  }!", f = 0.02f32);
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

