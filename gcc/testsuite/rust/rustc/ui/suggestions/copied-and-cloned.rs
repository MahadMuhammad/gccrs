//@ run-rustfix

fn expect<T>(_: T) {}

struct Issue114925 {
    x: Option<String>,
}

fn issue_114925(lol: &mut Issue114925, x: Option<&String>) {
    lol.x = x.clone();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

fn main() {
    let x = Some(&());
    expect::<Option<()>>(x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let x = Ok(&());
    expect::<Result<(), ()>>(x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let s = String::new();
    let x = Some(&s);
    expect::<Option<String>>(x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let x = Ok(&s);
    expect::<Result<String, ()>>(x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let s = String::new();
    let x = Some(s.clone());
    let y = Some(&s);
    println!("{}", x == y);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    //FIXME(#114050) ~| HELP use `Option::as_ref` to convert `Option<String>` to `Option<&String>`

    let mut s = ();
    let x = Some(s);
    let y = Some(&mut s);
    println!("{}", x == y);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let mut s = String::new();
    let x = Some(s.clone());
    let y = Some(&mut s);
    println!("{}", x == y);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    issue_114925(&mut Issue114925 { x: None }, None);
}

