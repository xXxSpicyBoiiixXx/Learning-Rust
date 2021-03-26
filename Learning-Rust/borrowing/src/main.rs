fn main() {
    
    let s1 = String::from("hello"); 
    
    /*
     * This is more of a formal way of doing things 
     * by printing the length. 
     */
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    /*
     * The issues with the tuple code above is that you have to 
     * return the String to the calling function so we can still use 
     * the String after the call to calculate_length, because the 
     * String was moved into calculate_length 
     *
     * Below is how you would define and use another version of 
     * calculate_length function that has a reference to an object
     * as a parameter instead of taking owndership of the value 
     */
    
    let s3 = String::from("hello");
    let len2 = calculate_length2(&s3); // Let's us create a reference that refers to the value of s3 but does not own it. 

    // The signature of the function uses `&` 

    println!("The length of '{}' is {}.", s3, len);
    
    /*
     * Mutable references
     * There is one big restriction 
     * You can only have one mutable 
     * reference to a particular piece of 
     * data in a particular scope.
     *
     * So something like this will fail 
     *
     * let mut s = String::from("hello") 
     *
     * let r1 = &mut s; 
     * let r2 = &mut s; 
     *
     * println!("{}, {}", r1, r2);
     *
     * This restriction allows for mutation but
     * in a controlled fashion. The prevents data 
     * races at compile time. A data race is similar to 
     * a race condtion and happens when three behaviors occur:
     *
     * 1) Two or more pointers access the same data at the same time
     * 2) At least one of the pointers is being used to write to the data
     * 3) There's no mechanism being used to synchronize access to the ata
     *
     * Rust prevents this cause it doesn't even compile code 
     * that has data races. 
     */
    let mut s4 = String::from("hello");

    change(&mut s4);

    /*
     * So you can still call multiple mutalable reference by 
     * creating new scope, but not simultaneous ones. 
     *
     * Another rule exists for combing mutable and immutable 
     * references. 
     * The below code results in an error. 
     *
     * let mut s = String::from("hello"); 
     *
     * let r1 = &s; // No problem 
     * let r2 = &s; // No problem 
     * let r3 = &mut s; // Problem 
     *
     * println!("{}, {}, and {}", r1, r2, r3);
     */

    /*
     * Reference's scope starts from where it is 
     * introduced and continues through the last 
     * time that reference is used. For instance the code 
     * below will compile because the last usage of the
     * immutable references occurs before the mutable reference
     * is introduce 
     */

    let mut s5 = String::from("hello");

    let r1 = &s5; //no problem 
    let r2 = &s5; //no problem
    println!("{} and {}", r1, r2);

    // Here, r1 and r2 are no longer used after this point 
    
    let r3 = &mut s5; // not problem 
    println!("{}", r3);

    /*
     * Rust won't allow any dangling pointers, 
     * so in turn you'll end up with a compiler-time error 
     * so if we have a reference to some data, the compiler 
     * will ensure that the data will not go out of scope 
     * before the reference to data does. If run the code below
     * it will create this compile-time error
     *
     * fn main() {
     * let reference_to_nothing = dangle();
     * }
     *
     * fn dangle() -> &String {  // dangle return a reference to a String 
     *
     * let s = String::from("hello"); // s is a new string  
     *
     * &s // We return a reference to the String, s 
     *
     * } // Here s goes out of the scope, and is dropped. It's memory goes away, so no. 
     *
     * The solution here is the return the String directly as its currently refercing nothing so
     * like the code below. 
     */

    let reference_to_nothing = no_dangle();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String 

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

// mutable reference function 
fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

// No dangling references 
// This works out since ownership moves out and nothing is deallocated.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
