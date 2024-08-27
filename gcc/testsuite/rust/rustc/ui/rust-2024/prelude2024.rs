//@ check-pass
//@ compile-flags: -Zunstable-options
// { dg-additional-options "-frust-edition=2024" }

fn main() {
    fut(async {}.into_future(), async {});
}

fn fut(_: impl Future, _: impl IntoFuture) {}

