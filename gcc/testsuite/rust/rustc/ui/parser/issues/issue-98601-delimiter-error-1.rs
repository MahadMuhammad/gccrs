fn foo() {
    match 0 {
      _ => {}
    }
    if foo
    }
} // { dg-error "" "" { target *-*-* } }

fn main() {}

