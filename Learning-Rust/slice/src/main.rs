fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s); // word will get the value 5

    let hello = &s[0..5];
    let world = &s[6..11];
    
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals 
    let word = first_word(&my_string_literal[..]);

    // Because string literal are string slices already, 
    // this works too, without the slice syntax. 
    let word = first_word(my_string_literal); 

    s.clear(); // This will empty the string, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. 
    // word is totally invalid now.
    
   // println!("The first word is: {}", word);
   
   // We can use slices for other things too....
   let a = [1, 2, 3, 4, 5];

   let slice = &a[1..3];
}

fn first_word(s: &str) -> &str { 
    let bytes = s.as_bytes(); // Converts the String variable into an                                 // array of bytes 

    for (i, &item) in bytes.iter().enumerate() { // Created an iterator for the bytes 
        if item == b' ' { 
           // return i;
           return &s[0..i];
        }
    }
    &s[..]
}

// fn second_word(s: &String) -> (usize, usize) {


