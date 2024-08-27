fn add_state(op: <isize as HasState>::State) {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

trait HasState {
    type State;
}

fn main() {}

