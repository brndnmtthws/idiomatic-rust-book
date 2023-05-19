// #![feature(trace_macros)]

macro_rules! noop_macro {
    () => {};
}

macro_rules! print_what_it_is {
    () => {
        println!("A macro with no arguments")
    };
    ($e:expr) => {
        println!("A macro with an expression")
    };
    ($s:stmt) => {
        println!("A macro with a statement")
    };
    ($e:expr, $s:stmt) => {
        println!("An expression followed by a statement")
    };
}

macro_rules! special_println {
    ($($arg:tt)*) => {
        println!("Printed specially: {}", format!($($arg)*))
    };
}

macro_rules! var_print {
    ($($v:ident),*) => {
        println!(concat!($(stringify!($v),"={:?} "),*), $($v),*)
    };
}

trait Dog {
    fn name(&self) -> &String;
    fn age(&self) -> i32;
    fn breed(&self) -> &String;
}

macro_rules! dog_struct {
    ($breed:ident) => {
        struct $breed {
            name: String,
            age: i32,
            breed: String,
        }
        impl $breed {
            fn new(name: &str, age: i32) -> Self {
                Self {
                    name: name.into(),
                    age,
                    breed: stringify!($breed).into(),
                }
            }
        }
        impl Dog for $breed {
            fn name(&self) -> &String {
                &self.name
            }
            fn age(&self) -> i32 {
                self.age
            }
            fn breed(&self) -> &String {
                &self.breed
            }
        }
    };
}

dog_struct!(Labrador);
dog_struct!(Golden);
dog_struct!(Poodle);

fn main() {
    noop_macro!();

    print_what_it_is!();
    print_what_it_is!({});
    print_what_it_is!(;);
    print_what_it_is!({}, ;);
    // print_what_it_is!(;, ;); error!

    // trace_macros!(true);
    special_println!("hello world!");
    // trace_macros!(false);

    special_println!("with an argument of {}", 5);

    let counter = 7;
    let gauge = 3.14;
    let name = "Peter";
    var_print!(counter, gauge, name);

    let peter = Poodle::new("Peter", 7);
    println!(
        "{} is a {} of age {}",
        peter.name(),
        peter.breed(),
        peter.age()
    );
}
