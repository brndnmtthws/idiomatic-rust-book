pub trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

#[derive(Debug)]
struct Bicycle {
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

struct BicycleBuilder {
    bicycle: Bicycle,
}

macro_rules! with_str {
    ($name:ident, $func:ident) => {
        pub fn $func(self, $name: &str) -> Self {
            Self {
                bicycle: Bicycle {
                    $name: $name.into(),
                    ..self.bicycle
                },
            }
        }
    };
}

macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        pub fn $func(self, $name: $type) -> Self {
            Self {
                bicycle: Bicycle {
                    $name,
                    ..self.bicycle
                },
            }
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
    let bicycle = Bicycle::builder()
        .with_make("Trek")
        .with_model("Madone")
        .with_size(52)
        .with_colour("purple")
        .build();
    println!("{:?}", bicycle);

    let bicycle1 = Bicycle {
        make: "Rivendell".into(),
        model: "A. Homer Hilsen".into(),
        size: 51,
        colour: "red".into(),
    };
    println!("{:?}", bicycle1);
    let bicycle2 = Bicycle {
        size: 58,
        ..bicycle1
    };
    println!("{:?}", bicycle2);
    // println!("{:?}", bicycle1); error! value used after move
}
