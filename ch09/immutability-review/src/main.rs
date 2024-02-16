use std::cell::RefCell;

fn mutability(
    a: i32,     // immutable
    mut b: i32, // mutable
) {
    // a += 1; // error: cannot assign twice to immutable variable `a`
    b += 1;

    dbg!(a);
    dbg!(b);
}

fn main() {
    let x = 1;
    dbg!(x);
    let y = x + 1; // y = 2
    dbg!(y);

    // x += 1; // error: cannot assign twice to immutable variable `x`

    let mut x = x; // x = 1
    x += 1; // x = 2
    dbg!(x);

    let a = 1;
    let b = 2;
    mutability(a, b);

    let immutable_string = String::from("This string cannot be changed");
    // immutable_string.push_str("... or can it?"); // error: cannot borrow
    // `immutable_string` as mutable, as it is not declared as mutable
    dbg!(&immutable_string);

    let not_so_immutable_string = RefCell::from(immutable_string);
    not_so_immutable_string
        .borrow_mut()
        .push_str("... or can it?");
    dbg!(&not_so_immutable_string);
}
