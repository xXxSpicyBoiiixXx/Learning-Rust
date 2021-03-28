#[derive(Debug)]
struct Rectangle { 
    width: u32,
    height: u32,
}

fn main() {
    
    // Without tuples 
    let width1 = 30; 
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels without tuple.",
        area(width1, height1)
        );

    // With tupes
    let rect1 = (30, 50); 

    println!(
        "The area of the rectangle is {} square pixels using a tuple.",
        area_tupes(rect1)
        );

    // Using a struct 
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels using a strucutre.", area_struct(&rect2)
        );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tupes(dimenstions: (u32, u32)) -> u32 {
    dimenstions.0 * dimenstions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
