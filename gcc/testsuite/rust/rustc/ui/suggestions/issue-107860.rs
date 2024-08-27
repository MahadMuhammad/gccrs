// { dg-additional-options "-frust-edition= 2021" }

async fn str<T>(T: &str) -> &str { &str }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn main() {}

