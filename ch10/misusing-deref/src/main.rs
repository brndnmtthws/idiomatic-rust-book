use std::ops::Deref;

struct Person(String, String, u32);

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Person {
    fn first_name_len(&self) -> usize {
        self.0.len()
    }
}

fn main() {
    let ferris = Person("Ferris".to_string(), "Bueller".to_string(), 17);
    println!("Hello, {}!", *ferris);
    println!("The length of a person is {}", ferris.len());

    println!(
        "The length of a person's first name is {}",
        ferris.first_name_len()
    );
}
