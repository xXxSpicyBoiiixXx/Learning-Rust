mod front_of_house {
	pub mod hosting { 
		// fn add_to_waitlist() {}
		
		pub fn add_to_waitlist() {} 

		fn seat_at_table() {}
	}

	pub mod serving {
		fn take_order() {}
	
		fn serve_order() {}
		
		fn take_payment() {}
	}
}

/*pub fn eat_at_restaurant() {
	
	// Absolute Path 
	crate::front_of_house::hosting::add_to_waitlist();
	
	// Relative Path 
	front_of_house::hosting::add_to_waitlist();
}*/

fn server_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order(); 
		super::server_order();
	}

	fn cook_order() {}

	pub struct Breakfast {
		pub toast: String, 
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}
	
	pub enum Appetizer {
		Soup, 
		Salad,
	}
}

use crate::front_of_house::hosting;

// You can also bring the crate into schope with the following command 
// use self::front_of_house::hosting; 

pub fn eat_at_restaurant() {
	
	let mut meal = back_of_house::Breakfast::summer("Rye");	
	meal.toast = String::from("Wheat");	
	println!("I'd like {} toast please", meal.toast);

	/* 
	 * The next line won't compile if we uncomment it, we're not allowed
	 * to see or modify the seasonal fruit that comes with the meal 
	 * meal.seasonal_fruit = String::from("blueberries");
	 */ 

	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;

	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
}


