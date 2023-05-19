pub trait SelfDescribing {
    fn describe() -> String;
}

fn describe_type<T: SelfDescribing>() -> String {
    T::describe()
}

struct Dog();
struct Cat();

impl SelfDescribing for Dog {
    fn describe() -> String {
        "happy little dog".into()
    }
}

impl SelfDescribing for Cat {
    fn describe() -> String {
        "curious cat".into()
    }
}

fn main() {
    println!("I am a {}", describe_type::<Dog>());
    println!("I am a {}", describe_type::<Cat>());
}
