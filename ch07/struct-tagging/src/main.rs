use std::marker::PhantomData;

trait BulbState {}

#[derive(Debug, Default)]
struct LightBulb<State: BulbState> {
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
struct On;
#[derive(Debug, Default)]
struct Off;

impl BulbState for On {}
impl BulbState for Off {}

impl LightBulb<On> {
    fn turn_off(self) -> LightBulb<Off> {
        LightBulb::<Off>::default()
    }
    fn state(&self) -> &str {
        "on"
    }
}

impl LightBulb<Off> {
    fn turn_on(self) -> LightBulb<On> {
        LightBulb::<On>::default()
    }
    fn state(&self) -> &str {
        "off"
    }
}

fn main() {
    let lightbulb = LightBulb::<Off>::default();
    println!("Bulb is {}", lightbulb.state());
    let lightbulb = lightbulb.turn_on();
    println!("Bulb is {}", lightbulb.state());
    let lightbulb = lightbulb.turn_off();
    println!("Bulb is {}", lightbulb.state());
}
