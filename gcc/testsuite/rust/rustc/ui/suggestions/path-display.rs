use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("/tmp/foo/bar.txt");
    println!("{}", path);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let path = PathBuf::from("/tmp/foo/bar.txt");
    println!("{}", path);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

