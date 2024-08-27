#![feature(type_alias_impl_trait)]

// { dg-additional-options "-frust-edition=2021" }

use std::future::Future;

struct Connection {
}

trait Transaction {
}

struct TestTransaction<'conn> {
    conn: &'conn Connection
}

impl<'conn> Transaction for TestTransaction<'conn> {
}

struct Context {
}

type TransactionResult<O> = Result<O, ()>;

type TransactionFuture<'__, O> = impl '__ + Future<Output = TransactionResult<O>>;
// { dg-error "" "" { target *-*-* } .-1 }

fn execute_transaction_fut<'f, F, O>(
// { dg-error "" "" { target *-*-* } .-1 }
    f: F,
) -> impl FnOnce(&mut dyn Transaction) -> TransactionFuture<'_, O>
where
    F: FnOnce(&mut dyn Transaction) -> TransactionFuture<'_, O> + 'f
{
    f
// { dg-error ".E0792." "" { target *-*-* } .-1 }
}

impl Context {
    async fn do_transaction<O>(
// { dg-error "" "" { target *-*-* } .-1 }
        &self, f: impl FnOnce(&mut dyn Transaction) -> TransactionFuture<'_, O>
    ) -> TransactionResult<O>
    {
// { dg-error ".E0792." "" { target *-*-* } .-1 }
// { dg-error ".E0792." "" { target *-*-* } .-2 }
        let mut conn = Connection {};
        let mut transaction = TestTransaction { conn: &mut conn };
        f(&mut transaction).await
    }
}

fn main() {}

