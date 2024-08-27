// #107983 - testing that `__iterator_get_unchecked` isn't suggested
// HELP included so that compiletest errors on the bad suggestion
pub fn i_can_has_iterator() -> impl Iterator<Item = u32> {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
// { help ".E0271." "" { target *-*-* } .-2 }
    Box::new(1..=10) as Box<dyn Iterator>
// { dg-error ".E0191." "" { target *-*-* } .-1 }
// { help ".E0191." "" { target *-*-* } .-2 }
}

fn main() {}

