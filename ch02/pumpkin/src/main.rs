use std::fmt::Debug;

#[derive(Clone, Debug)]
struct Pumpkin {
    mass: f64,
    diameter: f64,
}

impl Default for Pumpkin {
    fn default() -> Self {
        Self {
            mass: 2.0,
            diameter: 5.0,
        }
    }
}

fn main() {
    let big_pumpkin = Pumpkin {
        mass: 50.,
        diameter: 50.,
    };
    println!("Big pumpkin: {:?}", big_pumpkin);
    println!("Cloned big pumpkin: {:?}", big_pumpkin.clone());
    println!("Default pumpkin: {:?}", Pumpkin::default());
}
