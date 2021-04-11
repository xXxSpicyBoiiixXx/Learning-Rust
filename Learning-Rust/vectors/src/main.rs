enum SpreadsheetCell {
	Int(i32),
	Float(f64), 
	Text(String), 
}

/*let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.13),
];*/

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
	println!("{:?}", does_not_exist);

	/*
	let mut v = vec![1, 2, 3, 4, 5]; 
	
	let first = &v[0];
	
	v.push(6);
	
	println!("The first element is: {}", first); 
	
	*/
	
	// Iterating with a for loop 
	let v_it = vec![100, 32, 57]; 
	for i in &v_it {
		println!("{}", i);
	}
	
	// Iterating with a mutable references 
	let mut v_mut = vec![100, 32, 57];
	for i in &mut v_mut { 
		*i += 50; 
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.22),
	];
} // <----- v and v1 goes out of scope here and is freed
