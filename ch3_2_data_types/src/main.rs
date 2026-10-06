use std::io;

fn main() {

    // let x = 2.0; // f64 - default
    // let y: f32 = 3.0; // f32
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // fixed length, same data type

    // let a [3, 5] // same as let a = [3, 3, 3, 3, 3]


    // deliberately go out of array bounds to force panic.
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Indexed entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
