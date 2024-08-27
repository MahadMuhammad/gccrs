//@ compile-flags: --test

#![deny(unused_imports)]

use std::io::BufRead; // { dg-error "" "" { target *-*-* } }

fn a() {}
fn b() {}

mod test {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod tests {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod test_a {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod a_test {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod tests_a {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod a_tests {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod fastest_search {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

#[cfg(test)]
mod test_has_attr {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

mod test_has_no_attr {
    #[cfg(test)]
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b; // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {}

