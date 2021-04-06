#[derive(Debug)]
enum UsStates { 
	Alabama, 
	Alaska, 
	Texas, 
 	// The rest of the states... 	
}

enum Coin {	
	Penny,
	Nickel,
	Dime,
	Quarter,
	Quarter1(UsStates), 
}

fn value_in_cents(coin: Coin) -> u8 {

/*
 * The match type might seem like a another 
 * if statement, the difference here is that 
 * if statements only return boolean values 
 * while the match statement will return
 * any type. Teh type coin here is the "Coin" 
 * enum that we defined. 
 */
	match coin { 
	Coin::Penny => {
		println!("Lucky Penny!"); 
		1
	}
	Coin::Nickel => 5, 
	Coin::Dime => 10,
	Coin::Quarter => 25,
	Coin::Quarter1(state) => {
		println!("State quarter from {:?}!", state);
		25 
	}
}

}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
			None => None,
			Some(i) => Some(i + 1), 
		}
}

fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	let some_u8_value = 0u8;
	match some_u8_value { 
	1 => println!("one");
	3 => println!("three");
	5 => println!("five");
	7 => println!("seven");
	_ => (),
	}

	let some_u8_value1 = Some(0u8);
	match some_u8_value1 {
		Some(3) => println!("three"),
		_ => (),
		}
	
	/*
	 * The syntax "if let: take a pattern and an expression separated 
	 * by an equal sign. It works the same way as a match, where the 
	 * expression is given to the match and the pattern is its first 
	 * arm. 
	 * 
	 * Using if let means less typing, less indentation, and less biolerplate
	 * code. However, you lose the exhaustive checking that match enforces.
	 * Choosing between match and if let depends on what you're doing in your particualt 
	 * situation and whether gaining conciseness is an appropriate trade-off
	 * for losing exhausitve checking. 
	 * 
	 * In other wordsm you can think of "if let" as syntax sugar for a 
	 * match that runs code when the value matches one pattern and then ignores 
	 * all other values. 
	 * 
	 * WE can include an "else" with an "if let". The block of code that goes with the 
	 * "else" is the same as the vlock of code that would go with the "_" case 
	 * in the match expression that is equivalen to the "if let" 
	 */
 
	let some_u8_value2 = Some(0u8);
	if let Some(3) = some_u8_value {
		println!("three");
		}	
	
	let mut count = 0;
	match coin { 
		Coin::Quarter(state) => println!("State quarter from {:?}!", state), 
		_ => count += 1,
	}
	
	let mut count = 0;
	if let Coin::Quarter(state) = coin {
			println!("State quarter from {:?}!", state);
	} else {
		count += 1;
	}

