fn main() {
	
	// Empty String 
	let mut s1 = String::new();
	
	// Using a variable "data" to input to
	// a string as well 
	let data = "initial contents"; 
	let s2 = data.to_string();
	
	// Using a literal to put into the string s 
	let s3 = "initial contents".to_string();
	
	// Using the function String::from to put the content in
	let s4 = String::from("initial contents");

	// We can use any form of languages for the code as well 
	// Since this is UTF-8 encoded.
	

	/*
	 * UPDATING A STRING 
	 */ 
	
	let mut s5 = String::from("foo");
	s5.push_str("bar");

	// After the above code s5 will be foobar. 
	// Below we will see that the push_str method takes 
	// a string slice because we don't necessarily want 
	// to take ownership of the parameter. This isn't the 
	// case since hte push_str method doesn't 
	// take ownership 

	let mut s6 = String::from("foo");
	let s7 = "bar";
	s6.push_str(s7);
	println!("s7 is {}", s7);
	
	// Below is the push method 
	// It takes a single character as an input and adds it to the string 
	// such that it will add it to the end of the string, 
	// The below output will result in 'lol' 
	
	let mut s8 = String::from("lo");
	s8.push('l');

	// Often times we want to combine two strings, we can do this
	// with the '+' operator. 
	
	let s9 = String::from("Hello, "); 
	let s10 = String::from("world!");
	let s11 = s9 + &s10; // note s9 has been moved here adn can no longer be used. 
	
	/*
	 * We used the plus method with the reference as the function looks like below. 
	 * 
	 * fn add(self, s: &str) -> String { //some code} 
	 */ 	

	// In the above expresssion, s11 will take ownership of s9 and then appends 
	// a copy of the contents of s10, and then reutnr the ownership of the resulting 
	// in this case s11. 
	
	// Adding multiple things... 
	let s12 = String::from("tic");
	let s13 = String::from("tac");
	let s14 = String::from("toe");

	let s15 = s12 + "-" + &s13 + "-" + &s14;

	// For more complicated string combing, we use the format! macro: 
	let s16 = String::from("tic");
	let s17 = String::from("tac"); 
	let s18 = String::from("toe");
	
	let s19 = format!("{}-{}-{}", s16, s17, s18); 
	
	let hello = String::from("Hola");
	let idk = String::from("Je ne Sais Pa");
	let hello_ru = String::from("Привет"); 
	
	// We can't referecnce this below to avoid 
	// returning an unexpected balue and usually the 
	// user usually doesn't want byte values 
	// so Rust will aboid this by not compiling it at all 
	// and prevent misunderstaning early in development 
	// process. 
	// 
	// let answer = &hello_ru[0]; 
	
	
	/*
	 * Another reason Rust doesn't allow us to index into a String 
	 * to get a character is that indexing operations are expected 
	 * to always take constant time (O(1)). 
	 */ 

	// Using the code below to make Rust panic at runtime in the same as 
	// if an invalid index were accessed in a vector: 
	// let s = &hello_ru[0..4]; 
	
	// Be careful using ranges to create string slices with caution cause it 
	// will crash our program. 
	
	// The best way to do this is by using the "chars" method. So lets call "chars" 
	// as shown below 
	
	for c in "Иди нахой сука блять".chars() {
		println!("{}", c);
	}
	
	// We can do the same for the bytes methods here and see it! 
	
	for b in "Иди нахой сука блять".bytes() {
		println!("{}", b);
	}

	println!("This compiles"); 
}

