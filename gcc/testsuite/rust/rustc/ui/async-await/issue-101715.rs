// { dg-additional-options "-frust-edition=2018" }

struct S;

impl S {
    fn very_long_method_name_the_longest_method_name_in_the_whole_universe(self) {}
}

async fn foo() {
    S.very_long_method_name_the_longest_method_name_in_the_whole_universe()
        .await
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
}

fn main() {}

