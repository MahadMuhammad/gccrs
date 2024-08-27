//@ check-pass

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait IntFactory {
    fn stream(&self) -> impl Iterator<Item = i32>;
}
trait SendIntFactory: IntFactory<stream(..): Send> + Send {}

fn main() {}

