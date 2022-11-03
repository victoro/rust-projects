mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    // pub mod at_restaurant {
    //     pub fn eat() {
    //         self::hosting::add_to_waitlist();
    //     }
    // }
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

fn deliver_order() {}

// use crate::back_of_house;
use crate::back_of_house::Breakfast;
use back_of_house::Appetizer;
use front_of_house::hosting;
use front_of_house::serving;

//testing 'use' scope

mod customer {
    use crate::back_of_house::Appetizer;
    // use crate::back_of_house::Breakfast;
    use super::back_of_house::Breakfast;
    pub fn just_eating() {
        let meal = Breakfast::summer("rye");
        let appetizer1 = Appetizer::Soup;
    }
}
fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
    // making use of the 'use' statement
    serving::take_order();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");
    dbg!(meal);

    // making use of the 'use' statement
    let appetizer1 = Appetizer::Salad;
    //relative use of enum
    let appetizer2 = back_of_house::Appetizer::Soup;
}
