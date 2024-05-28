trait Animal {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}
impl Dog {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl Animal for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}
impl Cat {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl Animal for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let dog = Box::new(Dog::new("Rusty"));
    let cat = Box::new(Cat::new("Misty"));

    let animals: Vec<Box<dyn Animal>> = vec![dog, cat];

    for animal in animals {
        println!("{} says {}", animal.name(), animal.speak());
    }
}
