fn main() {
    println!("Hello, world!");

    let mut count = 0;
    // loops
    let result = loop {
        // loop shall return value in Rust, Damn awesome!
        count = count + 1;
        println!("again!");
        if count == 10 {
            break count * 2; // use break to break the loop based on the condtion
        }
    };
    println!("The result of loop count: {}", result);

    // Condtional loops - While
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through collection with 'for'
    let num_list = [10, 20, 30, 40, 50];

    for num in num_list.iter() {
        println!("The value in num_list: {}", num);
    }
    println!("reversing num_list using for loop");
    // reversing using for loop
    for num in num_list.iter().rev() {
        println!("The value in num_list: {}", num);
    }
}
