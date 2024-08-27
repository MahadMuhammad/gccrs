// { dg-additional-options "-frust-edition=2021" }

async fn foo() {}

fn main() {
    std::mem::size_of_val(foo());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

