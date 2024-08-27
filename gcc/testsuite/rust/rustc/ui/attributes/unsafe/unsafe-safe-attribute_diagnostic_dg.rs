#[unsafe(diagnostic::on_unimplemented( // { dg-error "" "" { target *-*-* } }
    message = "testing",
))]
trait Foo {}

fn main() {}

