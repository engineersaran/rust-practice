fn main() {

    // Scalar
    let x = 5;
    //let mut x = 5;
    let spaces = "    ";
    println!("The value of x is: {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS: {}", MAX_POINTS);
    let spaces: usize = spaces.len();
    println!("The length: {}", spaces);
    let _guess: u32 = "42".parse().expect("Not a number!");
    let _index: u8 = 255;
    let x = 2.012;
    let y: f32 = 3.034;
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // result of numerical operation
    println!(
        "{},{},{},{},{}",
        sum, difference, product, quotient, remainder
    );

    // boolean
    let t = true;
    let f: bool = false;
    println!("{},{}", t, f);

    // char
    let c = 'z';
    let z = 'ℤ';
    let _heart_eyed_cat = '😻';
    let japanese = '日';
    println!("{}, {}, {}", z, c, japanese);

    // Compound - tuples - fixed size,  different data types
    let tup = (500, 6.4, 1);
    let (m, n, p) = tup;
    // destructuring
    println!("m: {}, n: {}, p: {}", m, n, p);
    // accessing by using index
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // Compund - arrays - fixed size, same data type
    let index = 10;
    let one = [1, 2, 3, 4, 5];
    let two: [i32; 5] = [1, 2, 3, 4, 5];
    let three = [3; 5];
    let four: [&str; 3] = ["Saran", "Mother", "Niha"];
    println!("{}, {}, {}, {}", one[0], three[1], four[2], two[3]);
    println!("{}, {}, {}, {}", one[0], three[1], four[index], two[3]);
}
