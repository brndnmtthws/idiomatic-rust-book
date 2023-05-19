fn main() {
    let bark = || println!("Bark!");
    bark();

    let increment = |value| value + 1;
    increment(1);

    let print_and_increment = |value| {
        println!("{value} will be incremented and returned");
        value + 1
    };
    print_and_increment(5);

    let left_value = || 1;
    let right_value = || 2;
    let adder = |left: fn() -> i32, right: fn() -> i32| left() + right();
    adder(left_value, right_value); // returns 3

    let consumable = String::from("cookie");
    let consumer = move || consumable;
    consumer();
    // consumer(); error!
}
