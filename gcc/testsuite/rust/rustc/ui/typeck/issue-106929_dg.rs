struct Client;

impl Client {
    fn post<T: std::ops::Add>(&self, _: T, _: T) {}
}

fn f() {
    let c = Client;
    post(c, ());
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() {}

