use std::io;

fn main() {
    
    // integer practice

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of integer x in the inner scope is: {x}");
    }

    println!("The value of integer x is : {x}");

    // floating point numbers
    let x2 = 2.0; // f64
    let y2: f32 = 3.0; // f32

    println!("My floating point number examples are {x2} and {y2}");

    // numeric operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Rust can add 5+10={sum} 95.5-4.3={difference} 4*30={product} 56.7/32.2={quotient} -5/3={truncated} 43 % 5={remainder}");

    // boolean practice
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("Boolean t is {t} and f is {f}");

    // character practice
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Character examples are {c} {z} and {heart_eyed_cat}");

    // tuple practice

    // destructuring
    let tup = (500, 6.4, 1);
    let (x3, y3, z3) = tup;
    println!("The value of y is: {y3}");

    // indexing with .0 .1 .2 etc
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("Tuple Practice: {x3} {y3} {z3} and {five_hundred} {six_point_four} {one}");

    // array practice

    // array example
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let march = months[2];
    println!("The third month is {march}");

    // let c: [i32; 5] = [1, 2, 3, 4, 5]; type;number of elements in the array
    // let a = [3; 5] is the same as: a = [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

/*
Scalar Types

Integers: number with no fractional component, signed if it can be negative (i32) and unsigned if it will always be positive (u32)
Signed: -(2n - 1) to 2n - 1 - 1 inclusive vs Unsigned: 0 to 2n - 1
Floating Point Numbers: f64 is default
Booleans: true or false
Characters: Unicode Scalar Value (ASCII, accented letters, emoji, zero width spaces are all included)

Compound Types:
Tuples can contain multiple variable types and they cannot grow or shrink
Arrays must all contain the same type and they must be a fixed length
Vectors can grow and shrink

*/