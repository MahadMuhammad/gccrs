mod child {
    trait Main {
        fn main() -> impl std::process::Termination;
    }

    struct Something;

    impl Main for () {
        fn main() -> Something {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
            Something
        }
    }
}

fn main() {}

