use std::marker::PhantomData;

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

enum Recursive<T> {
    Next(Box<Recursive<T>>),
    Boxed(Box<T>),
    Optional(Option<T>),
}

enum NextNode<T> {
    Next(Box<ListNode<T>>),
    End,
}

struct ListNode<T> {
    data: Box<T>,
    next: NextNode<T>,
}

struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

impl Dog<Labrador> {
    fn breed_name(&self) -> &str {
        "labrador"
    }
}
impl Dog<Retriever> {
    fn breed_name(&self) -> &str {
        "retriever"
    }
}
impl Dog<Poodle> {
    fn breed_name(&self) -> &str {
        "poodle"
    }
}
impl Dog<Dachshund> {
    fn breed_name(&self) -> &str {
        "dachshund"
    }
}

fn main() {
    let str_container = Container { value: "&str" };
    println!("{}", str_container.value);
    let ambiguous_container: Container<Option<String>> =
        Container { value: None };
    let alt_ambiguous_container: Container<Option<String>> =
        Container::new(None);
    let short_alt_ambiguous_container =
        Container::<Option<String>>::new(None);

    let my_poodle: Dog<Poodle> = Dog {
        name: "Jeffrey".into(),
        breed: PhantomData,
    };
    println!(
        "My dog is a {}, named {}",
        my_poodle.breed_name(),
        my_poodle.name,
    );
}
