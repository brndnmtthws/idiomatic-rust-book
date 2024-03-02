use std::ops::Deref;

struct Name(String);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let name = Name("Ferris".to_string());
    println!("Hello, {}!", *name);
    println!("The length of the name is {}", name.len());
}
