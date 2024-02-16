use rpds::Vector;

fn main() {
    let streets = Vector::new()
        .push_back("Elm Street")
        .push_back("Maple Street")
        .push_back("Oak Street");

    let updated_streets = streets.push_back("Pine Street");

    dbg!(&streets);
    dbg!(&updated_streets);
}
