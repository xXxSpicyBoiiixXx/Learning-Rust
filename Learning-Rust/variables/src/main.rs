fn main() {
    
    /*
    let mut x = 5;
    println!("The value of x is: {}", x); 
    x = 6;
    println!("The value of x is: {}", x);
    */

    let x = 5; 

    let x = x + 1; 

    let x = x * 2; 

    println!("The value of x is: {}", x);


    /* 
     * This is allowed since we have the first "spaces" 
     * variable as a string, while we intilize a 
     * second "spaces" variables as a number type 
     */
    let spaces = "  "; 
    let spaces = spaces.len();

    /*
     * This is not allowed since we are are not 
     * allowed to mutate a variable type. If we 
     * try to run this with cargo run, we will 
     * get a error that says we're not allowed to 
     * mutate a variable's type. 
     */

    // let mut spaces = "   ";
    // spaces = spaces.len();
    
    let x_f1 = 2.0; // f64

    let x_f2: f32 = 3.0; //f32 

    let sum = 5 + 10; 
    let difference = 95.5 - 4.3;
    let product = 4* 30; 
    let quotient = 56.7/32.2; 
    let remainder = 43 % 5; 

    let t = true; 
    let f: bool = false; // with explicit type annotation

    let c = 'z'; 

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    
    let a = [1,2,3,4,5];

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];



}
