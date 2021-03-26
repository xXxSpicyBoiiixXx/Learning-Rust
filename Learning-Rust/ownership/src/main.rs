fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5; 
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("Hello"); // s comes into scope 
    
    takes_ownership(s); // s's value moves int othe function 
                        // so this is where it's no longer 
                        // valid 
    
    let x_scope = 5;    // x comes into scope 

    makes_copy(x_scope); // x_scope would move into the
                         // function, but i32 is Copy, 
                         // so it's okay to use 
                         // x_scope afterwards. 
    
    let s1_own = gives_ownership(); // gives_ownership move 
                                    // it's return 
                                    // value into s1_own
    
    let s2_own = String::from("hello"); // s2_own comes into scope

    let s3_own = takes_and_gives_back(s2_own); // s2_own is moved into takes_and_gives_backm which also 
                                               // moves its return value into s3_own 

} // Here, x_scope goes out of scope, then s. But since, s's value 
  // has moved nothing really happens here.
  // In additon s3_own goes out of scope and is dropped
  // s2_own goes out of scope but was moved
  //

fn takes_ownership(some_string: String) { // Some_string comes into scope 
    println!("{}", some_string); 
} // Here, some_string goes out of scope and "drop" is called.
  // So the memeroy is free

fn makes_copy(some_integer: i32) { // some_integer comes into scope  
    println!("{}", some_integer); 
} // some_integer goes out of scope
