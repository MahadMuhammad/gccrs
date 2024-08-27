fn iter<T>(vec: Vec<T>) -> impl Iterator<Item = T> {
    vec.into_iter()
}
fn foo() {
    let vec = vec!["one", "two", "three"];
    while let Some(item) = iter(vec).next() { // { dg-error ".E0382." "" { target *-*-* } }
// { help ".E0382." "" { target *-*-* } .-1 }
        println!("{:?}", item);
    }
}
fn bar() {
    let vec = vec!["one", "two", "three"];
    loop {
// { help "" "" { target *-*-* } .-1 }
        let Some(item) = iter(vec).next() else { // { dg-error ".E0382." "" { target *-*-* } }
            break;
        };
        println!("{:?}", item);
    }
}
fn baz() {
    let vec = vec!["one", "two", "three"];
    loop {
// { help "" "" { target *-*-* } .-1 }
        let item = iter(vec).next(); // { dg-error ".E0382." "" { target *-*-* } }
// { help ".E0382." "" { target *-*-* } .-1 }
        if item.is_none() {
            break;
        }
        println!("{:?}", item);
    }
}
fn qux() {
    let vec = vec!["one", "two", "three"];
    loop {
// { help "" "" { target *-*-* } .-1 }
        if let Some(item) = iter(vec).next() { // { dg-error ".E0382." "" { target *-*-* } }
            println!("{:?}", item);
            break;
        }
    }
}
fn zap() {
    loop {
        let vec = vec!["one", "two", "three"];
        loop {
// { help "" "" { target *-*-* } .-1 }
            loop {
                loop {
                    if let Some(item) = iter(vec).next() { // { dg-error ".E0382." "" { target *-*-* } }
                        println!("{:?}", item);
                        break;
                    }
                }
            }
        }
    }
}
fn main() {
    foo();
    bar();
    baz();
    qux();
    zap();
}

