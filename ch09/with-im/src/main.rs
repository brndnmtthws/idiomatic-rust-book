use im::vector;

fn main() {
    let shopping_list =
        vector!["milk", "bread", "butter", "cheese", "eggs"];

    let mut updated_shopping_list = shopping_list.clone();
    updated_shopping_list.push_back("grapes");

    dbg!(&shopping_list);
    dbg!(&updated_shopping_list);
}
