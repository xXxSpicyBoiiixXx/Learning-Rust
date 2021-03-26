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

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String 

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}
