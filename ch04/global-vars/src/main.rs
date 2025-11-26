use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use static_init::dynamic;
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

thread_local! {
    static POPULAR_BABY_NAMES_2021: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
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
    POPULAR_BABY_NAMES_2021.with(|rc| {
        rc.borrow_mut().extend([
            "Olivia".to_string(),
            "Liam".to_string(),
            "Emma".to_string(),
            "Noah".to_string(),
        ]);
    });

    let popular_baby_names_2017: OnceCell<Vec<String>> = OnceCell::new();
    popular_baby_names_2017.get_or_init(|| {
        vec![
            String::from("Emma"),
            String::from("Liam"),
            String::from("Olivia"),
            String::from("Noah"),
        ]
    });

    let names = POPULAR_BABY_NAMES_2021.with(|rc| rc.borrow().clone());
    println!("popular baby names of 2021: {:?}", names);

    println!("popular baby names of 2020: {:?}", *POPULAR_BABY_NAMES_2020);

    println!("popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);

    println!("popular baby names of 2018: {:?}", *POPULAR_BABY_NAMES_2018);

    println!(
        "popular baby names of 2017: {:?}",
        popular_baby_names_2017.get()
    );
}
