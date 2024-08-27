pub fn print_values(values: &impl IntoIterator)
where {
    for x in values.into_iter() {
        println!("{x}");
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}

fn main() {}

