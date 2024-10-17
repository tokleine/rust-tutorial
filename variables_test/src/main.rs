// use std::io;

fn main() {
    // Mutability & Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2; // this x SHADOWS the outer x; it is only 12 _inside_ this scope
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let y: i32 = 8;
    {
        let mut y = y;
        y += 1;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    // Scalar Types
    let z = 98_222;
    println!("The value of z is: {z}");
    let z = 0xff;
    println!("The value of z binary 0xff is: {z}");
    let z = 0o77;
    println!("The value of z octal 0o77 is: {z}");
    let z = 0b1111_0000;
    println!("The value of z binary 0b1111_0000 is: {z}");
    let z = b'A';
    println!("The value of z byte b'A' is: {z}");

    let mut z: u8 = 255;
    z = z.saturating_add(5);
    println!("The value of z saturates at: {z}");
    
    let mut z: u8 = 255;
    z = z.wrapping_add(1);
    println!("The value of z wraps if exceeding 255: {z}");

    let z: u8 = 255;
    let overflowed = z.overflowing_add(1).1;
    println!("The value of z overflows beyond 255: {overflowed}");

    let z: u8 = 255;
    let result = z.checked_add(1);
    match result {
        Some(value) => println!("The value of z is: {}", value),
        None => println!("Overflow occurred.")
    }
    println!("The value of z is still: {z}");

    // Floats
    let x = 2.0; // f64 double precision
    println!("The value of x is: {x}");

    let y: f32 = 3.0; // f32 single precision
    println!("The value of y is: {y}");

    // Numeric Operations
    let truncated: f32 = 8_f32 / 3_f32;
    println!("The float value of 8 / 3 is: {truncated}");

    let truncated = 8 / 3;
    println!("The integer value of 8 / 3 is: {truncated}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup; // destructuring
    let _five_hundred = tup.0; // access by index
    let _tup = (); // empty tuple, is always returned when a function returns nothing

    // arrays - static size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0]; // access by index
    let _a = [3; 5]; // [3, 3, 3, 3, 3]


    // access array element by index
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let mut index = String::new();

    // println!("Please enter an array index.");
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Please type a number!"); // providing an out-of-bounds index will cause a panic

    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    // Functions
    print_labeled_measurement(5,'m');


    // Ownership & Scope
    // __stack__ - Last-in-first-out (like a pile of dishes)
    {
        let x = 5;
        let y = x; // y is a copy of x and both are allocated in the stack memory and available in the same scope
        println!("The value of x in the stack memory is: {x}");
        println!("The value of y in the stack memory is: {y}");
    }

    // __heap__ - big chunk of memory that is accessed via pointers, the pointers are on the stack
    {
        let s1 = String::from("hello");
        let s2 = s1; // the head data is untouched, only the pointer is moved, and s1 is no longer valid
        // println!("The value of s1 in the heap memory is: {s1}"); // this will cause an error because s1 has been moved to s2
        println!("The value of s2 in the heap memory is: {s2}");
    }

    // heap - deep copies va clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // this creates a deep copy of the heap data
        println!("The value of s1 in the heap memory is: {s1}");
        println!("The value of s2 in the heap memory is: {s2}");
    }

    // Statements & Expressions
    // statements are instructions that perform some action and do not return a value
    // expression are instructions that return a value
    let x = 5; // statement
    let _y = { // expression
        x + 1
    };
    println!("The value of y is: {_y}");


    // Functions do not need a return statement, the last expression is returned
    let _x = five();
    println!("The value of x from the five function is: {_x}");



}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}