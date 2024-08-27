fn foo() {
    let foos = vec![String::new()];
    let bars = vec![""];
    let mut baz = vec![];
    let mut qux = vec![];
    for foo in foos { for bar in &bars { if foo == *bar {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
        baz.push(foo);
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        continue;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    } }
    qux.push(foo);
// { dg-error ".E0382." "" { target *-*-* } .-1 }
// { dg-note ".E0382." "" { target *-*-* } .-2 }
    }
}

fn main() {
    let foos = vec![String::new()];
    let bars = vec![""];
    let mut baz = vec![];
    let mut qux = vec![];
    for foo in foos {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        for bar in &bars {
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
            if foo == *bar {
                baz.push(foo);
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
                continue;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            }
        }
        qux.push(foo);
// { dg-error ".E0382." "" { target *-*-* } .-1 }
// { dg-note ".E0382." "" { target *-*-* } .-2 }
    }
}

