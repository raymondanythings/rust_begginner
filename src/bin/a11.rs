// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity = {:?}", item.quantity)
}

fn display_id(item: &GroceryItem) {
    println!("id = {:?}", item.id)
}

fn main() {
    let my_item = GroceryItem {
        id: 1,
        quantity: 30,
    };

    display_id(&my_item);
    display_quantity(&my_item);
}
