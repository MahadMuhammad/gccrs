mod a {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);

    impl Debug for Inner<()> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>); // { dg-error ".E0277." "" { target *-*-* } }
}

mod b {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);

    impl<T: Debug> Debug for Inner<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>);
}

mod c {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);
    trait Trait {}

    impl<T: Debug + Trait> Debug for Inner<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>); // { dg-error ".E0277." "" { target *-*-* } }
}

mod d {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);
    trait Trait {}

    impl<T> Debug for Inner<T> where T: Debug, T: Trait {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>); // { dg-error ".E0277." "" { target *-*-* } }
}

mod e {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);
    trait Trait {}

    impl<T> Debug for Inner<T> where T: Debug + Trait {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>); // { dg-error ".E0277." "" { target *-*-* } }
}

mod f {
    use std::fmt::{Debug, Formatter, Result};
    struct Inner<T>(T);
    trait Trait {}

    impl<T: Debug> Debug for Inner<T> where T: Trait {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            todo!()
        }
    }

    #[derive(Debug)]
    struct Outer<T>(Inner<T>); // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

