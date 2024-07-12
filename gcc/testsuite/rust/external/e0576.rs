#![allow(unused)]
fn main() {
trait Hello {
    type Who;

    fn hello() -> <Self as Hello>::You; // error!
}
}