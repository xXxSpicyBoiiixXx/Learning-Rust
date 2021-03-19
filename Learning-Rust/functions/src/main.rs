fn main() {

    println!("Hello, world!");

    another_function();

    another_function1(5);

    another_function2(5, 6);

    let i = 5;

    let j = {
        let i = 3; 
        i + 1
    };

    println!("The value of j is: {}", j);

    let five_fn = five();

    println!("The value of five_fn is: {}", five_fn);

    let add_one = plus_one(5);

    println!("The value of add_one is: {}", add_one);
}

fn another_function() {
    println!("Another function");
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
