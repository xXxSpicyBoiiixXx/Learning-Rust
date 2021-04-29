// Abstraction of our find largest value. 
fn largest(list: &[i32]) -> &i32 {
	
	let mut largest = &list[0];
	
	for item in list { 
		if item > largest {
			largest = item; 
		}
	}
	
	largest 
}

fn main() {

/*	// Listing a list of numbers and then we place
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
	
	// Prints out the largest number in the data set
	// This should be 100 from our list 
	println!("The largest number is {}", largest);
*/
	// To find the largest number in two different lists of numbers,
	// we can duplicate the code from above and use the same 
	// logic in two different places 

/*	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8]; 
	
	let mut largest = number_list[0];

	for number in number_list {
		if number > largest {
			largest = number; 
		}
	}

	println!("The largest number is {}", largest);

*/

	/*
	 * Now it would be lame to really duplicate code like this 
	 * and this is also prone to error. To elimnante this 
	 * duplication, we can create an abstraction by definining a function 
	 * that operation on a list of varibles. This solution makes our 
	 * code clearer and let's us express the concept of finding the largest 
	 * number in a list abstractly. 
	 * 
	 * We made a function called largest on top of the main function. 
	 */ 

	// Repeating the previous code but now using our function 

	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest(&number_list);
	println!("The largest number is {}", result); 
	
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8]; 
	
	let result = largest(&number_list); 
	println!("The largest number is {}", result);
}
