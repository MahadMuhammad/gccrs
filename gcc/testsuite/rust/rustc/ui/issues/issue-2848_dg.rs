#[allow(non_camel_case_types)]

mod bar {
    pub enum foo {
        alpha,
        beta,
        charlie
    }
}

fn main() {
    use bar::foo::{alpha, charlie};
    match alpha {
      alpha | beta => {} // { dg-error ".E0170." "" { target *-*-* } }
// { dg-error ".E0170." "" { target *-*-* } .-1 }
      charlie => {}
    }
}

