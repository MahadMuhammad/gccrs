trait Tr: !SuperA {}
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr2: SuperA + !SuperB {}
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr3: !SuperA + SuperB {}
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr4: !SuperA + SuperB
// { dg-error "" "" { target *-*-* } .-1 }
+ !SuperC + SuperD {}
// { dg-error "" "" { target *-*-* } .-1 }
trait Tr5: !SuperA
// { dg-error "" "" { target *-*-* } .-1 }
+ !SuperB {}
// { dg-error "" "" { target *-*-* } .-1 }

trait SuperA {}
trait SuperB {}
trait SuperC {}
trait SuperD {}

fn main() {}

