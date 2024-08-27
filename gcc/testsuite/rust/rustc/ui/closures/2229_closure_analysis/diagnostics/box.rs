// { dg-additional-options "-frust-edition=2021" }

// Test borrow checker when we precise capture when using boxes

struct MetaData { x: String, name: String }
struct Data { m: MetaData }
struct BoxedData(Box<Data>);
struct EvenMoreBoxedData(Box<BoxedData>);

// Check diagnostics when the same path is mutated both inside and outside the closure
fn box_1() {
    let m = MetaData { x: format!("x"), name: format!("name") };
    let d = Data { m };
    let b = BoxedData(Box::new(d));
    let mut e = EvenMoreBoxedData(Box::new(b));

    let mut c = || {
        e.0.0.m.x = format!("not-x");
    };

    e.0.0.m.x = format!("not-x");
// { dg-error ".E0506." "" { target *-*-* } .-1 }
    c();
}

// Check diagnostics when a path is mutated inside a closure while attempting to read it outside
// the closure.
fn box_2() {
    let m = MetaData { x: format!("x"), name: format!("name") };
    let d = Data { m };
    let b = BoxedData(Box::new(d));
    let mut e = EvenMoreBoxedData(Box::new(b));

    let mut c = || {
        e.0.0.m.x = format!("not-x");
    };

    println!("{}", e.0.0.m.x);
// { dg-error ".E0502." "" { target *-*-* } .-1 }
    c();
}

// Check diagnostics when a path is read inside a closure while attempting to mutate it outside
// the closure.
fn box_3() {
    let m = MetaData { x: format!("x"), name: format!("name") };
    let d = Data { m };
    let b = BoxedData(Box::new(d));
    let mut e = EvenMoreBoxedData(Box::new(b));

    let c = || {
        println!("{}", e.0.0.m.x);
    };

    e.0.0.m.x = format!("not-x");
// { dg-error ".E0506." "" { target *-*-* } .-1 }
    c();
}

fn main() {
    box_1();
    box_2();
    box_3();
}

