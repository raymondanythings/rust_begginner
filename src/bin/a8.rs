// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drint(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("flavor : Sparkling"),
        Flavor::Sweet => println!("flavor : Sweet"),
        Flavor::Fruity => println!("flavor : Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.2,
    };
    print_drint(sweet);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 6.2,
    };
    print_drint(fruity);
}
