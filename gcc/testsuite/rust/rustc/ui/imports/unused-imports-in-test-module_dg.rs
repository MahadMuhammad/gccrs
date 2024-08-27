#![deny(unused_imports)]

use std::io::BufRead; // { dg-error "" "" { target *-*-* } }

fn a() {}
fn b() {}

mod test {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod tests {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod test_a {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod a_test {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod tests_a {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod a_tests {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

mod fastest_search {
    use super::a; // { dg-error "" "" { target *-*-* } }

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

#[cfg(test)]
mod test_has_attr {
    use super::a;

    #[test]
    fn foo() {
        a();
        use crate::b;
    }
}

fn main() {}

