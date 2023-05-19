use std::any::Any;

trait MyTrait {
    fn trait_hello(&self);
    fn as_any(&self) -> &dyn Any;
}

struct MyStruct1;

impl MyStruct1 {
    fn struct_hello(&self) {
        println!("Hello, world! from MyStruct1");
    }
}

struct MyStruct2;

impl MyStruct2 {
    fn struct_hello(&self) {
        println!("Hello, world! from MyStruct2");
    }
}

impl MyTrait for MyStruct1 {
    fn trait_hello(&self) {
        self.struct_hello();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyTrait for MyStruct2 {
    fn trait_hello(&self) {
        self.struct_hello();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut v = Vec::<Box<dyn MyTrait>>::new();

    v.push(Box::new(MyStruct1 {}));
    v.push(Box::new(MyStruct2 {}));

    v.iter().for_each(|i| i.trait_hello());
    // v.iter().for_each(|i| i.struct_hello());

    println!("With a downcast:");
    v.iter().for_each(|i| {
        if let Some(obj) = i.as_any().downcast_ref::<MyStruct1>() {
            obj.struct_hello();
        }
        if let Some(obj) = i.as_any().downcast_ref::<MyStruct2>() {
            obj.struct_hello();
        }
    });
}
