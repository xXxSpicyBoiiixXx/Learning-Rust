/*
 * Below is a bit probalamtic 
 *
 * use std::fmt::Result 
 * use std::io::Result 
 *
 * This is due to the fact that we ahve two "Result" types 
 * in the same scope and Rust wouldn''t know which one we 
 * meant when we used "Result" 
 * 
 * You can do the commented thing below. 
 */ 

// use std::fmt; 
// use std::io;

/*
fn function1() -> fmt::Result {
	// Some Code 
}

fn function2() -> fmt::Result {
	// Some Code
}

*/

// If we wanted to use Result as the word we can just use 
// what's below, so it won't conflict. 

use std::fmt::Result; 
use std::io::Result as IoResult;

fn function1() -> Result {
	// Some code 
}

fn function2() -> IoResult<()> {
	// Some code 
}
