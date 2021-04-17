use std::fs::File; 
use std::io::ErrorKind; 
use std::io;
use std::io::Read; 
use std::fs;

enum Result<T ,E> {
	Ok(T), 
	Err(E),
}


/* 
 * When we propagate an error, we can write a function such as below
 * this instead of handling the error, it will just return the error 
 * to the calling code so that it can decide what to do
 * 
 * The function belowm reads a username from a file. If the file
 * doesn't exists or can't be read, this function will return those
 * errors to the code that called this function. 
 */

fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello2.txt");
	
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	
	let mut s = String::new();
	
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}	
}

// A short for propagating errors: The ? operator 

/*
 * The ? placed after a Result value is defined to work in almost
 * the same way as the match expression we defined to handle the 
 * Result values. 
 */
fn read_username_from_file2() -> Result<String, io::Error> {
	let mut f = File::open("hello3.txt")?;
	let mut s = String::new(); 
	f.read_to_string(&mut s)?;
	Ok(s)
}

// An even shorter version of the function with the ? operator 

fn read_username_from_file3() -> Result<String, io::Error> {
	let mut s = String::new();
	
	File::open("hello4.txt")?.read_to_string(&mut s)?;
	
	Ok(s)
}

// An even short method 
fn read_username_from_file4() -> Result<String, io::Error> {
	fs::read_to_string("hello5.txt")
}

fn main() {

//	panic!("Crash and Burn Baby!");

	// Accessing an index out of bounds
	// This will case that the code gets run 
	// when we use [] on out vector v
//	 let v = vec![1, 2, 3];
//	 v[99];
	

	let f1 = File::open("hello.txt");
	
	// Below this will lead to mismatch type	
	// let f2: u32 = File::open("hello.txt");

	/* The below code will fail no matter what and won't say why 
	 * for instance this will panic! no matter why.
	 	
	let f1 = match f1 { 
		Ok(file) => file, 
		Err(error) => panic!("Problem opening the file: {:?}", error), 
	};

	*/
	
	/*
	 * This will handle it a bit differently from above, since this will 
	 * the standrd library std::io::ErrorKind; and there is a variant in this library 
	 * that we would like to use, "ErrorKind::NotFound" which indicates the file 
	 * that we are trying to open doesn't exists. When the file isn't found 
	 * we try to create it and if it can't be created than we can another error. 
 
	let f1 = match f1 { 
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc, 
				Err(e) => panic!("Problem creating the file: {:?}", e),
				},
				other_error => {
					panic!("Problem opening the file: {:?}", other_error)
				}
			},
		}; */

	// Now the match expression is very useful, but also very primitive. A more experinced coder would write the below
	
	let f1 = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| { 
				panic!("Problem creating the file: {:?}", error); 
			})
		} else {
			panic!("Problem opening the file {:?}", error);
		}
	});
	
	// Below we use expect in the same ways as unwrap: to return the file handle
	// or call the panic! macro. The error message used by expect in its call 
	// to panic! will be the parameter that we pass to expect, rather than the
	// default panic! message that unwrap uses. 

	let f3 = File::open("hello1.txt").expect("Failed to open hello1.txt");

	
}

