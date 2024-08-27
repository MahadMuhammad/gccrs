fn main() {
    let tiles = Default::default();
    for row in &mut tiles {
        for tile in row {
            *tile = 0;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { dg-note ".E0282." "" { target *-*-* } .-2 }
        }
    }

    let tiles: [[usize; 3]; 3] = tiles;
}

