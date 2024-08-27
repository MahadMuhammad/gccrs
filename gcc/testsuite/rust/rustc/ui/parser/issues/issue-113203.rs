// Checks what happens when we attempt to use the await keyword as a prefix. Span
// incorrectly emitted an `.await` in E0277 which does not exist
// { dg-additional-options "-frust-edition=2018" }
fn main() {
    await {}()
// { dg-error "" "" { target *-*-* } .-1 }
}

