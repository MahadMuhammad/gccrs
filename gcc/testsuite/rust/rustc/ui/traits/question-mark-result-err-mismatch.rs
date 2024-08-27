fn foo() -> Result<String, String> { // { dg-note "" "" { target *-*-* } }
    let test = String::from("one,two");
    let x = test
        .split_whitespace()
        .next()
        .ok_or_else(|| {
            "Couldn't split the test string"
        });
    let one = x
        .map(|s| ())
        .map_err(|e| { // { dg-note "" "" { target *-*-* } }
            e; // { help "" "" { target *-*-* } }
        })
        .map(|()| "")?; // { dg-error ".E0277." "" { target *-*-* } }
// { dg-note ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
// { dg-note ".E0277." "" { target *-*-* } .-5 }
// { dg-note ".E0277." "" { target *-*-* } .-6 }
    Ok(one.to_string())
}

fn bar() -> Result<(), String> { // { dg-note "" "" { target *-*-* } }
    let x = foo(); // { dg-note "" "" { target *-*-* } }
    let one = x
        .map(|s| ())
        .map_err(|_| ())?; // { dg-error ".E0277." "" { target *-*-* } }
// { dg-note ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
// { dg-note ".E0277." "" { target *-*-* } .-5 }
// { dg-note ".E0277." "" { target *-*-* } .-6 }
// { dg-note ".E0277." "" { target *-*-* } .-7 }
// { dg-note ".E0277." "" { target *-*-* } .-8 }
// { help ".E0277." "" { target *-*-* } .-9 }
    Ok(one)
}

fn baz() -> Result<String, String> { // { dg-note "" "" { target *-*-* } }
    let test = String::from("one,two");
    let one = test
        .split_whitespace()
        .next()
        .ok_or_else(|| { // { dg-note "" "" { target *-*-* } }
            "Couldn't split the test string"; // { help "" "" { target *-*-* } }
        })?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
// { dg-note ".E0277." "" { target *-*-* } .-5 }
// { dg-note ".E0277." "" { target *-*-* } .-6 }
// { dg-note ".E0277." "" { target *-*-* } .-7 }
    Ok(one.to_string())
}

fn main() {}

