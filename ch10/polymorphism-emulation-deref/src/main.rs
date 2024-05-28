use std::ops::Deref;

struct Animal {
    name: String,
}
impl Animal {
    fn new(name: &str) -> Animal {
        Animal {
            name: name.to_string(),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Dog(Animal);
impl Dog {
    fn new(name: &str) -> Self {
        Self(Animal::new(name))
    }
    fn speak(&self) -> &str {
        "Woof!"
    }
}
impl Deref for Dog {
    type Target = Animal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Cat(Animal);
impl Cat {
    fn new(name: &str) -> Self {
        Self(Animal::new(name))
    }
    fn speak(&self) -> &str {
        "Meow!"
    }
}
impl Deref for Cat {
    type Target = Animal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let dog = Dog::new("Rusty");
    let cat = Cat::new("Misty");

    println!("{} says: {}", dog.name(), dog.speak());
    println!("{} says: {}", cat.name(), cat.speak());
}
