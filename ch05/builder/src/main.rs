pub trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

#[derive(Debug)]
pub struct Bicycle {
    make: String,
    model: String,
    size: i32,
    colour: String,
}

impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}

macro_rules! accessor {
    ($name:ident, &$ret:ty) => {
        pub fn $name(&self) -> &$ret {
            &self.$name
        }
    };
    ($name:ident, $ret:ty) => {
        pub fn $name(&self) -> $ret {
            self.$name
        }
    };
}

impl Bicycle {
    accessor!(make, &String);
    accessor!(model, &String);
    accessor!(size, i32);
    accessor!(colour, &String);
}

pub trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

pub struct BicycleBuilder {
    bicycle: Bicycle,
}

macro_rules! with_str {
    ($name:ident, $func:ident) => {
        pub fn $func(&mut self, $name: &str) {
            self.bicycle.$name = $name.into()
        }
    };
}

macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        pub fn $func(&mut self, $name: $type) {
            self.bicycle.$name = $name
        }
    };
}

impl BicycleBuilder {
    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(colour, with_colour);
}

impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                colour: String::new(),
            },
        }
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}

fn main() {
    let mut bicycle_builder = Bicycle::builder();
    bicycle_builder.with_make("Huffy");
    bicycle_builder.with_model("Radio");
    bicycle_builder.with_size(46);
    bicycle_builder.with_colour("red");
    let bicycle = bicycle_builder.build();
    println!("My new bike: {:?}", bicycle);
}
