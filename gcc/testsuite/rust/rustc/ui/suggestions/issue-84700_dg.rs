// test for suggestion on fieldless enum variant

#[derive(PartialEq, Debug)]
enum FarmAnimal {
    Worm,
    Cow,
    Bull,
    Chicken { num_eggs: usize },
    Dog (String),
}

fn what_does_the_animal_say(animal: &FarmAnimal) {

    let noise = match animal {
        FarmAnimal::Cow(_) => "moo".to_string(),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        FarmAnimal::Chicken(_) => "cluck, cluck!".to_string(),
// { dg-error ".E0164." "" { target *-*-* } .-1 }
        FarmAnimal::Dog{..} => "woof!".to_string(),
        _ => todo!()
    };

    println!("{:?} says: {:?}", animal, noise);
}

fn main() {}

