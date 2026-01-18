fn main() {
    // compound data types examples
    //array, tuple, struct, enum, slices, strings

    //array
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Number array: {:?}", arr);

    let fruits: [&str; 4] = ["apple", "banana", "cherry", "date"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array: {:?}", fruits[2]);

    //tuple
    let person: (&str, i32, f64) = ("Alice", 30, 65.5);
    println!("Person tuple: {:?}", person);
    println!("Name: {}, Age: {}, Weight: {}", person.0, person.1, person.2);

    let tupple_nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.5, 4.5));
    println!("Nested tuple: {:?}", tupple_nested);

    let mix_type_tuple: (&str, i32, bool, f64, char, [i32; 3]) = ("Bob", 25, true, 70.0, 'M', [1,2,3]);
    println!("Mixed type tuple: {:?}", mix_type_tuple);


    //slic
    // contains a reference to a contiguous sequence of elements in a collection
    let arr_slice: &[i32; 5] = &[10, 20, 30, 40, 50];
    //let slice: &[i32] = &arr
    println!("Array slice: {:?}", arr_slice);

    let animal_slice: &[&str] = &["cat", "dog", "rabbit"];
    println!("Animal slice: {:?}", animal_slice);

    let book_slice: &[&String] = &[&"The Rust Programming Language".to_string(), &"Rich dad poor dad".to_string(), &"Atomic habits".to_string()];
    println!("Book slice: {:?}", book_slice);


    //strings vs string slices
    //strings are growable, heap-allocated data structures, mutable owned by the String type.
    let  mut greeting: String = String::from("Hello");
    println!("Greeting string: {}", greeting);
    greeting.push_str(", world!");
    println!("Updated greeting string: {}", greeting);

     //string slices are references to a sequence of UTF-8 encoded characters.
     // memory efficient, immutable views into a string.
    let str_string: String = String::from("Hello, Rust!");
    let str_slice: &str = &str_string[0..5];
    println!("String slice: {}", str_slice);


    //struct
    struct Car {
        make: String,
        model: String,
        year: u16,  
    }
    let car1 = Car {
        make: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2020,
    };
    let car2 = Car {
        make: String::from("Honda"),
        model: String::from("Civic"),
        year: 2019,
    };
    println!("Car: {} {} {}", car1.make, car1.model, car1.year);
    println!("Car: {} {} {}", car2.make, car2.model, car2.year);


    //enum
    //enums are used to define a type that can be one of several variants.
    //each variant can have different data associated with it.
    //example
    enum Shape {
        Circle(f64),          // radius
        Rectangle(f64, f64),  // width, height
        Square(f64),          // side length
        triangle(f64, f64, f64), // sides a, b, c
    }
    let shape1 = Shape::Circle(3.0);
    let shape2 = Shape::Rectangle(4.0, 5.0);
    let shape3 = Shape::Square(2.5);
    let shape4 = Shape::triangle(3.0, 4.0, 5.0);
    match shape2 {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
        Shape::Square(side) => println!("Square with side length: {}", side),
        Shape::triangle(a, b, c) => println!("Triangle with sides: {}, {}, {}", a, b, c),
    }



}