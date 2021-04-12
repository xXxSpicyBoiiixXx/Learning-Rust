use std::collections::HashMap;

fn main() {
    	
	// Creates an empty hash value
	let mut scores = HashMap::new(); 
	
	// adding elements into out function with "insert"
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	// Another way of constructing a hash map is by using 
	// iterators and the "collect" method on a vector of 
	// tuples, where each tuple consists of a key 
	// and its value. 
	
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10,50]; 
	
	// For the parameter for the key and value types, however
	// we use underscores, and Rust can inger the types that 
	// the hash map contains based on the types of the data in the 
	// vectors
		
	let mut scoring: HashMap<_, _> = 
		teams.into_iter().zip(initial_scores.into_iter()).collect();

	let field_name = String::from("Fav Color"); 
	let field_value = String::from("Blue");
	
	let mut map = HashMap::new(); 
	map.insert(field_name, field_value); 
	
	// we aren't able to use field_name or field_value after moving 
	// them into the hash map with the call to "insert" 

	/*
	 * If we insert references to values into the hash map, 
	 * the values won't be moved into the hash map. 
	 * The values that the reerences point to must be valid for at least
	 * as long as the hash map is valid.  
	 */ 
	
	// Below we can access values out of the hash map by providing its key to the 
	// 'get" method 

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
	
	// Here, score will have the value of Some(&10) 
	// the result is wrapped in "Some" because "get" returns an "Option<&V>" 
	// If there are no values for that key in the hash map, "get" will return
	// "None". 

	// We can also iterate over each ley/value pair in a hash map in 
	// a similar manner as we do with vectors with a for loop. 
	
	for(key,value) in &scores {
		println!("{}: {}", key, value);
	}

	// Each key can only have one value associated with it. 
println!("Compiles");

}
