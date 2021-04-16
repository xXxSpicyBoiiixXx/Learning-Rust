use std::fs::File; 
use std::io::ErrorKind; 

enum Result<T ,E> {
	
	Ok(T), 
	Err(E),

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
		};
}

