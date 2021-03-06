// Loads module contents from another file with the same name as the module
pub mod hosting;

// All module items (functions, structs, enums, etc.) are private by default
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

// since this is a sibling of the private front_of_house module, it can use it
pub fn eat_at_restaurant() {
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // won't compile! "field `seasonal_fruit` of struct `back_of_house::Breakfast` is private"
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    let order1 = crate::back_of_house::Appetizer::Soup;
    let order2 = crate::back_of_house::Appetizer::Salad;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Courtesy of `use` above
    // Note: It's idiomatic when bring a function into a scope to
    // use the parent module rather than the function itself to
    // avoid confusion about where the function is declared.
    hosting::add_to_waitlist();

    // For structs, enums, and other items it's idiomatic to
    // specify the full path as demonstrated here.
    let mut map = HashMap::new();
    map.insert(1, 2);

    fn function1() -> Result { Result::Ok(()) }
    fn function2() -> IoResult<()> { IoResult::Ok(()) }
}