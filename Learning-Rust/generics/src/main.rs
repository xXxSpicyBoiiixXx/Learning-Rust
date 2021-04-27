fn main() {

	// Listing a list of numbers and then we place
	// the first numebr in the list inside the 
	// variable `largest`	
	let number_list = vec![34, 50, 25, 100, 65];
	let mut largest = number_list[0];
	
	// Then we iterate through all the numbers 
	// in the list, and if the current number 
	// is greater then we replace it.
	for number in number_list {
		if number > largest {
			largest = number; 
		} 
	}

	println!("The largest number is {}", largest);
}
