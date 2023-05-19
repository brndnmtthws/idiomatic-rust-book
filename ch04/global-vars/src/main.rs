use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use static_init::dynamic;
use std::sync::{Arc, Mutex};

thread_local! {
    static POPULAR_BABY_NAMES_2021: Arc<Mutex<Option<Vec<String>>>> =
        Arc::new(Mutex::new(None));
}

lazy_static! {
    static ref POPULAR_BABY_NAMES_2020: Vec<String> = {
        vec![
            String::from("Olivia"),
            String::from("Liam"),
            String::from("Emma"),
            String::from("Noah"),
        ]
    };
}

static POPULAR_BABY_NAMES_2019: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        String::from("Olivia"),
        String::from("Liam"),
        String::from("Emma"),
        String::from("Noah"),
    ]
});

#[dynamic]
static POPULAR_BABY_NAMES_2018: Vec<String> = vec![
    String::from("Emma"),
    String::from("Liam"),
    String::from("Olivia"),
    String::from("Noah"),
];

fn main() {
    let arc = POPULAR_BABY_NAMES_2021.with(|arc| arc.clone());
    let mut inner = arc.lock().expect("unable to lock mutex");
    *inner = Some(vec![
        String::from("Olivia"),
        String::from("Liam"),
        String::from("Emma"),
        String::from("Noah"),
    ]);

    println!("popular baby names of 2021: {:?}", *inner);

    println!("popular baby names of 2020: {:?}", *POPULAR_BABY_NAMES_2020);

    println!("popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);

    println!("popular baby names of 2018: {:?}", *POPULAR_BABY_NAMES_2018);
}
