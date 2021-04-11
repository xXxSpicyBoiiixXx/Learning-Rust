fn main() {
    	// Below is a type annotation, but isn't realistic 
	// let v: Vec<i32> = Vec::new();

	// more relistic code is below since Rust can infer the type 
	
	let v = vec![1, 2, 3, 4, 5];

	// Updating a vector
	
	let mut v1 = Vec::new();
	v1.push(5);
	v1.push(6);
	v1.push(7);
	v1.push(8);
	
	let thrid: &i32 = &v[2]; 
	println!("The thrid element is {}", thrid); 
	
	match v.get(2) {
		Some(thrid) => println!("The third element is {}", thrid),	
		None => println!("There is no thrid element"),
	}

	// let does_not_exisst = &v[100]; <--- This will cause the program 
	// to panic because its regerences a nonecistent element. 

	let does_not_exist = v.get(100); // <---- returns "None" 

} // <----- v and v1 goes out of scope here and is freed
