impl<T> T { // { dg-error ".E0118." "" { target *-*-* } }
    fn get_state(&self) -> String {
       String::new()
    }
}

fn main() {}

