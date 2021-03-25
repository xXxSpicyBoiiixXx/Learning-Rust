fn main() {

    /*
     * Prints out that the condition is true
     */
    let number = 3; 

    if number < 5 { 
        println!("Condition was true"); 
    } else { 
        println!("Condition was false");
    }
    
    /*
     * Prints out that the condition is false
     */
    let number1 = 7;
    
    if number1 < 5 {
        println!("Condition was true");
    } else { 
        println!("Condition was false");
    }
    
    /*
     * Checks if the number is something other than zero
     */
    let number2 = 10; 

    if number2 != 0 { 
        println!("Number was something other than zero!");
    }

    /*
     * Checks what the number maybe divisiable by in orders of 4, 5, or 6. 
     * So there is a problem here, 5 is divisible by 2. We don't see the other 
     * statements being ran so this creates a problem. This is due to that 
     * Rust will only executes the block for the first true condition, and once 
     * it find ones, it doesn't check the rest. So be careful of using too many *else if*
     * statements. So we may want to use a refractor if we have more than one. 
     */

    let number3 = 6;

    if number3 % 4 == 0 {
        println!("Number is divisible by 4"); 
    } else if number3 % 3 == 0 {
        println!("Number is divisible by 3"); 
    } else if number3 % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("NUmber is not divisible by 4, 3, or 2"); 
        }

    /*
     * So since "if" is an expression, we can use it on the right side of the "let" statement 
     */
    let condition = true; 
    let number = if condition { 5 } else { 6 }; 

    println!("The value of number is: {}", number);

   /*
    * Returning values from loops 
    * Below is a loop that will return the result of the counring which is 20. 
    * This is useful to check if the thread has completed 
    */

    let mut counter = 0; 
    
    let result = loop {
        counter += 1; 

        if counter == 10 {
            break counter * 2;
        }

    };

    println!("The result is {}", result); 

    /*
     * Conditional loops with while statements be like lmfao 
     * This eliminates a lot of nesting that would be needed if we used 
     * the other consturcts. 
     */

    let mut while_number = 3; 

    while while_number != 0 {
        println!("{}!", while_number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

}
