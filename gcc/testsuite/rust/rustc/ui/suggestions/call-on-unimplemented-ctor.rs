fn main() {
    insert_resource(Marker);
    insert_resource(Time);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
}

trait Resource {}

fn insert_resource<R: Resource>(resource: R) {}

struct Marker;
impl Resource for Marker {}

struct Time(u32);

impl Resource for Time {}

