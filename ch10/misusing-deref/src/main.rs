use std::ops::Deref;

struct Person(String, String, u32);

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let ferris = Person("Ferris".to_string(), "Bueller".to_string(), 17);
    println!("Hello, {}!", *ferris);
    println!("The length of a person is {}", ferris.len());
}
