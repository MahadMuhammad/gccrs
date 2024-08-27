struct TestClient;

impl TestClient {
    fn get_inner_ref(&self) -> &Vec<usize> {
        todo!()
    }
}

fn main() {
    let client = TestClient;
    let inner = client.get_inner_ref();
// { help "" "" { target *-*-* } .-1 }
    inner.clear();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-note ".E0596." "" { target *-*-* } .-2 }
}

