use std::borrow::Cow;

#[derive(Debug, Clone)]
struct CowList<'a> {
    cows: Cow<'a, [String]>,
}

impl<'a> CowList<'a> {
    fn add_cow(&self, cow: &str) -> Self {
        let mut new_cows = self.clone();
        new_cows.cows.to_mut().push(cow.to_string());
        new_cows
    }
}

impl Default for CowList<'_> {
    fn default() -> Self {
        CowList {
            cows: Cow::from(vec![]),
        }
    }
}

#[derive(Debug, Clone)]
struct CowVec<'a> {
    cows: Vec<Cow<'a, str>>,
}

fn loud_moo<'a>(mut cow: Cow<'a, str>) -> Cow<'a, str> {
    if cow.contains("moo") {
        Cow::from(cow.to_mut().replace("moo", "MOO"))
    } else {
        cow
    }
}

fn main() {
    let cow_say_what = Cow::from("The cow goes moo");
    dbg!(&cow_say_what);

    let cows_dont_say_what =
        cow_say_what.clone().to_mut().replace("moo", "toot");
    dbg!(&cow_say_what);
    dbg!(&cows_dont_say_what);

    let yelling_cows = loud_moo(cow_say_what.clone());
    dbg!(&cow_say_what);
    dbg!(&yelling_cows);

    let list_of_cows = CowList::default()
        .add_cow("Bessie")
        .add_cow("Daisy")
        .add_cow("Moo");
    dbg!(&list_of_cows);
    let list_of_cows_plus_one = list_of_cows.add_cow("Penelope");
    dbg!(&list_of_cows);
    dbg!(&list_of_cows_plus_one);
}
