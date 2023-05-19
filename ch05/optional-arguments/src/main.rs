struct Container {
    name: String,
}

trait First {
    fn name(&self) {}
}

trait Second {
    fn name(&self, _: bool) {}
}

impl First for Container {
    fn name(&self) {}
}

impl Second for Container {
    fn name(&self, _: bool) {}
}

fn get_name_from_first<T: First>(t: &T) {
    t.name()
}
fn get_name_from_second<T: Second>(t: &T) {
    t.name(true)
}

fn main() {
    let container = Container {
        name: "Henry".into(),
    };
    get_name_from_first(&container);
    get_name_from_second(&container);
}
