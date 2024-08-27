struct Node_ {
    a: Box<Cycle>
}

enum Cycle {
    Node(Node_),
    Empty,
}

fn main() {
    let mut x: Box<_> = Box::new(Cycle::Node(Node_ {a: Box::new(Cycle::Empty)}));

    // Create a cycle!
    match *x {
      Cycle::Node(ref mut y) => {
        y.a = x; // { dg-error ".E0505." "" { target *-*-* } }
      }
      Cycle::Empty => {}
    };
}

