// { dg-additional-options "-frust-edition=2018" }

async fn print_dur() {}

fn main() {
    async { let (); }.await;
// { dg-error ".E0728." "" { target *-*-* } .-1 }
    async {
        let task1 = print_dur().await;
    }.await;
// { dg-error ".E0728." "" { target *-*-* } .-1 }
    (|_| 2333).await;
// { dg-error ".E0728." "" { target *-*-* } .-1 }
}

