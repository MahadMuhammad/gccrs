fn main() {}
trait StreamOnce {
    type Position;
}
impl StreamOnce for &str {
    type Position = usize;
}
fn follow(_: &str) -> <&str as StreamOnce>::Position {
    String::new  // { dg-error ".E0308." "" { target *-*-* } }
}

