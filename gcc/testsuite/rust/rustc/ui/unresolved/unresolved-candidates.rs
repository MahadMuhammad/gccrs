mod a {
    pub trait Trait {}
}

mod b {
    use Trait; // { dg-error ".E0432." "" { target *-*-* } }
}

mod c {
    impl Trait for () {} // { dg-error ".E0405." "" { target *-*-* } }
}

fn main() {}

