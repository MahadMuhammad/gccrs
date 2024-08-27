//@ incremental
// { dg-additional-options "-frust-edition=2021" }

fn main() {
    let _ = async {
        let s = std::array::from_fn(|_| ()).await;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    };
}

