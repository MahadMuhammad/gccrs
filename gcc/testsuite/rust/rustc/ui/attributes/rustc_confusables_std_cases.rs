use std::collections::BTreeSet;
use std::collections::VecDeque;

fn main() {
    let mut x = BTreeSet::new();
    x.push(1); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
    let mut x = Vec::new();
    x.push_back(1); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
    let mut x = VecDeque::new();
    x.push(1); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
    let mut x = vec![1, 2, 3];
    x.length(); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
    x.size(); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    x.append(42); // { dg-error ".E0308." "" { target *-*-* } }
// { help ".E0308." "" { target *-*-* } .-1 }
    String::new().push(""); // { dg-error ".E0308." "" { target *-*-* } }
// { help ".E0308." "" { target *-*-* } .-1 }
    String::new().append(""); // { dg-error ".E0599." "" { target *-*-* } }
// { help ".E0599." "" { target *-*-* } .-1 }
}

