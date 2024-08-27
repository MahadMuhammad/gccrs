//@ run-rustfix

fn get_vowel_count(string: &str) -> usize {
    string
        .chars()
        .filter(|c| "aeiou".contains(c))
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        .count()
}

fn main() {
    let _ = get_vowel_count("asdf");
}

