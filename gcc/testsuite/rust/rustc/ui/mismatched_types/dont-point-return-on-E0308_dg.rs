// { dg-additional-options "-frust-edition=2021" }

async fn f(_: &()) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// Second note is the span of the underlined argument, I think...

fn main() {
    (|| async {
        Err::<(), ()>(())?;
        f(());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { help ".E0308." "" { target *-*-* } .-4 }
        Ok::<(), ()>(())
    })();
}

