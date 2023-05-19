struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

struct Square {
    length: i32,
}

impl Square {
    pub fn new(length: i32) -> Self {
        Self { length }
    }
    pub fn get_length(&self) -> i32 {
        self.length
    }
}

pub trait Rectangular {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_area(&self) -> i32;
}

impl Rectangular for Rectangle {
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_area(&self) -> i32 {
        self.width * self.height
    }
}

impl Rectangular for Square {
    fn get_width(&self) -> i32 {
        self.length
    }
    fn get_height(&self) -> i32 {
        self.length
    }
    fn get_area(&self) -> i32 {
        self.length * self.length
    }
}

fn main() {
    let rect = Rectangle::new(2, 3);
    let square = Square::new(5);

    println!(
        "rect has width {}, height {}, and area {}",
        rect.get_width(),
        rect.get_height(),
        rect.get_area()
    );
    println!(
        "square has length {} and area {}",
        square.get_length(),
        square.get_area()
    );
}
