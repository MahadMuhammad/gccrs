// { dg-additional-options "-frust-edition= 2021" }

fn main() {}

async fn foo() {
    None { value: (), ..Default::default() }.await;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

